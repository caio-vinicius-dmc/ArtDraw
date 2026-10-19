//! Independent vector PDF 1.4 export. Model pixels use 96 dpi (0.75 points/pixel).
use crate::document::{Document, Page};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    path::Path,
};
use vello::kurbo::{BezPath, PathEl, Point};
const LIMIT: usize = 64 * 1024 * 1024;
fn num(value: f64) -> Result<String, String> {
    if !value.is_finite() || value.abs() > 1e9 {
        return Err("PDF: coordenada inválida ou acima do limite de1 bilhão".into());
    }
    let mut s = format!("{value:.8}");
    while s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    if s == "-0" {
        s = "0".into();
    }
    Ok(s)
}
fn values(v: impl IntoIterator<Item = f64>) -> Result<String, String> {
    v.into_iter()
        .map(num)
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.join(" "))
}
fn paint(c: [u8; 4]) -> Result<String, String> {
    values(c[..3].iter().map(|&v| f64::from(v) / 255.))
}
fn path(out: &mut String, path: &BezPath) -> Result<(), String> {
    let mut current = Point::ZERO;
    let mut start = Point::ZERO;
    for element in path.elements() {
        match *element {
            PathEl::MoveTo(p) => {
                writeln!(out, "{} m", values([p.x, p.y])?).unwrap();
                current = p;
                start = p;
            }
            PathEl::LineTo(p) => {
                writeln!(out, "{} l", values([p.x, p.y])?).unwrap();
                current = p;
            }
            PathEl::QuadTo(q, p) => {
                let a = current + (q - current) * (2. / 3.);
                let b = p + (q - p) * (2. / 3.);
                writeln!(out, "{} c", values([a.x, a.y, b.x, b.y, p.x, p.y])?).unwrap();
                current = p;
            }
            PathEl::CurveTo(a, b, p) => {
                writeln!(out, "{} c", values([a.x, a.y, b.x, b.y, p.x, p.y])?).unwrap();
                current = p;
            }
            PathEl::ClosePath => {
                out.push_str("h\n");
                current = start;
            }
        }
        if out.len() > LIMIT {
            return Err("PDF excede64MiB".into());
        }
    }
    Ok(())
}
#[derive(Default)]
struct Writer {
    objects: Vec<Vec<u8>>,
    size: usize,
}
impl Writer {
    fn reserve(&mut self) -> usize {
        self.objects.push(Vec::new());
        self.objects.len()
    }
    fn set(&mut self, id: usize, data: impl Into<Vec<u8>>) -> Result<(), String> {
        let data = data.into();
        self.size = self.size.checked_add(data.len()).ok_or("PDF excessivo")?;
        if self.size > LIMIT {
            return Err("PDF excede64MiB".into());
        }
        self.objects[id - 1] = data;
        Ok(())
    }
    fn add(&mut self, data: impl Into<Vec<u8>>) -> Result<usize, String> {
        let id = self.reserve();
        self.set(id, data)?;
        Ok(id)
    }
    fn finish(self, root: usize, info: usize) -> Result<Vec<u8>, String> {
        let mut out = b"%PDF-1.4\n%\xD3\xEB\xE9\xE1\n".to_vec();
        let mut offsets = vec![0];
        for (index, object) in self.objects.iter().enumerate() {
            if object.is_empty() {
                return Err("Objeto PDF incompleto".into());
            }
            offsets.push(out.len());
            out.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
            out.extend_from_slice(object);
            out.extend_from_slice(b"\nendobj\n");
        }
        let xref = out.len();
        out.extend_from_slice(
            format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len()).as_bytes(),
        );
        for offset in offsets.iter().skip(1) {
            out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        out.extend_from_slice(format!("trailer\n<< /Size {} /Root {root} 0 R /Info {info} 0 R >>\nstartxref\n{xref}\n%%EOF\n",offsets.len()).as_bytes());
        if out.len() > LIMIT {
            return Err("PDF excede64MiB".into());
        }
        Ok(out)
    }
}
fn page_stream(
    page: &Page,
    writer: &mut Writer,
    alpha: &mut BTreeMap<(u8, u8), usize>,
) -> Result<(String, BTreeSet<usize>), String> {
    if !(1.0..=100000.).contains(&page.width) || !(1.0..=100000.).contains(&page.height) {
        return Err("PDF: dimensões de página inválidas".into());
    }
    page.validate_groups()?;
    let layer_ids: BTreeSet<_> = page.layers.iter().map(|l| l.id).collect();
    if layer_ids.len() != page.layers.len()
        || page.objects.iter().any(|o| !layer_ids.contains(&o.layer))
    {
        return Err("PDF: referências de camada inválidas".into());
    }
    let mut out = format!(
        "q\n0.75 0 0 -0.75 0 {} cm\n1 1 1 rg\n0 0 {} {} re f\n0 0 {} {} re W n\n",
        num(page.height * 0.75)?,
        num(page.width)?,
        num(page.height)?,
        num(page.width)?,
        num(page.height)?
    );
    let mut resources = BTreeSet::new();
    for layer in &page.layers {
        if !layer.visible {
            continue;
        }
        for object in page
            .objects
            .iter()
            .filter(|o| o.layer == layer.id && o.visible)
        {
            if !object.stroke_width.is_finite() || object.stroke_width < 0. {
                return Err("PDF: espessura inválida".into());
            }
            let fill = object.fill.is_some();
            let stroke = object.stroke.is_some() && object.stroke_width > 0.;
            if !fill && !stroke {
                continue;
            }
            let pair = (
                object.fill.map_or(255, |c| c[3]),
                object.stroke.map_or(255, |c| c[3]),
            );
            let gs = if let Some(&id) = alpha.get(&pair) {
                id
            } else {
                let id = writer.add(format!(
                    "<< /Type /ExtGState /ca {} /CA {} /BM /Normal >>",
                    num(f64::from(pair.0) / 255.)?,
                    num(f64::from(pair.1) / 255.)?
                ))?;
                alpha.insert(pair, id);
                id
            };
            resources.insert(gs);
            writeln!(
                out,
                "q\n{} cm\n/GS{gs} gs\n1 J 1 j",
                values(object.transform.as_coeffs())?
            )
            .unwrap();
            if let Some(c) = object.fill {
                writeln!(out, "{} rg", paint(c)?).unwrap();
            }
            if stroke {
                writeln!(
                    out,
                    "{} RG\n{} w",
                    paint(object.stroke.unwrap())?,
                    num(object.stroke_width)?
                )
                .unwrap();
            }
            path(&mut out, &object.local_path())?;
            out.push_str(match (fill, stroke, object.evenodd) {
                (true, true, true) => "B*\n",
                (true, true, false) => "B\n",
                (true, false, true) => "f*\n",
                (true, false, false) => "f\n",
                _ => "S\n",
            });
            out.push_str("Q\n");
        }
    }
    out.push_str("Q\n");
    Ok((out, resources))
}
pub fn encode(doc: &Document, all_pages: bool) -> Result<Vec<u8>, String> {
    let active = doc
        .pages
        .iter()
        .find(|p| p.id == doc.active_page)
        .ok_or("PDF: página ativa ausente")?;
    if doc.pages.len() > 1000 {
        return Err("PDF: limite de páginas excedido".into());
    }
    let pages: Vec<_> = if all_pages {
        doc.pages.iter().collect()
    } else {
        vec![active]
    };
    let mut w = Writer::default();
    let catalog = w.reserve();
    let tree = w.reserve();
    let mut page_ids = Vec::new();
    let mut alpha = BTreeMap::new();
    for page in pages {
        let id = w.reserve();
        page_ids.push(id);
        let (content, resources) = page_stream(page, &mut w, &mut alpha)?;
        let length = content.len();
        let stream = w.add(format!(
            "<< /Length {length} >>\nstream\n{content}endstream"
        ))?;
        let gs = resources
            .iter()
            .map(|id| format!("/GS{id} {id} 0 R"))
            .collect::<Vec<_>>()
            .join(" ");
        w.set(id,format!("<< /Type /Page /Parent {tree} 0 R /MediaBox [0 0 {} {}] /Resources << /ExtGState << {gs} >> >> /Group << /Type /Group /S /Transparency /CS /DeviceRGB /I true >> /Contents {stream} 0 R >>",num(page.width*0.75)?,num(page.height*0.75)?))?;
    }
    w.set(
        tree,
        format!(
            "<< /Type /Pages /Count {} /Kids [{}] >>",
            page_ids.len(),
            page_ids
                .iter()
                .map(|id| format!("{id} 0 R"))
                .collect::<Vec<_>>()
                .join(" ")
        ),
    )?;
    w.set(catalog, format!("<< /Type /Catalog /Pages {tree} 0 R >>"))?;
    let title = active
        .name
        .encode_utf16()
        .map(|v| format!("{v:04X}"))
        .collect::<String>();
    let info = w.add(format!(
        "<< /Creator (ArtDraw 1.9) /Producer (ArtDraw vector PDF) /Title <FEFF{title}> >>"
    ))?;
    w.finish(catalog, info)
}
pub fn save(file: &Path, doc: &Document, all_pages: bool) -> Result<(), String> {
    if !file
        .extension()
        .and_then(|v| v.to_str())
        .is_some_and(|v| v.eq_ignore_ascii_case("pdf"))
    {
        return Err("Use a extensão .pdf para exportar".into());
    }
    let bytes = encode(doc, all_pages)?;
    crate::storage::atomic_write(file, &bytes, true)
}
/// Authored color/geometry corpus used only by the explicit PDF QA commands.
pub fn proof_document() -> Document {
    use crate::document::{Geometry, Layer, Object};
    use vello::kurbo::{Affine, Rect, Shape};
    let mut d = Document::with_size(800., 600.);
    d.name = "Prova PDF - ação".into();
    d.add_object(Object::new(
        "Vermelho",
        Geometry::Rectangle(Rect::new(60., 60., 240., 220.)),
        Some([255, 0, 0, 255]),
        None,
        0.,
    ));
    d.objects[0].locked = true;
    d.add_object(Object::new(
        "Azul50%",
        Geometry::Rectangle(Rect::new(140., 100., 320., 260.)),
        Some([0, 0, 255, 128]),
        None,
        0.,
    ));
    let mut curve = BezPath::new();
    curve.move_to((400., 80.));
    curve.curve_to((450., 10.), (650., 250.), (700., 80.));
    d.add_object(Object::new(
        "Cúbica",
        Geometry::Compound(curve),
        None,
        Some([0, 130, 30, 192]),
        8.,
    ));
    let mut holes = Rect::new(400., 300., 700., 540.).to_path(0.1);
    holes.extend(
        Rect::new(470., 370., 630., 470.)
            .to_path(0.1)
            .elements()
            .iter()
            .copied(),
    );
    let i = d.add_object(Object::new(
        "Furo evenodd",
        Geometry::Compound(holes),
        Some([255, 180, 0, 255]),
        Some([0, 0, 0, 255]),
        2.,
    ));
    d.objects[i].evenodd = true;
    d.add_object(Object::new(
        "Texto Unicode",
        Geometry::Text {
            position: Point::new(50., 330.),
            text: "Olá, ação!\nCurvas vetoriais".into(),
            size: 28.,
        },
        Some([30, 30, 30, 255]),
        None,
        0.,
    ));
    d.layers.push(Layer {
        id: 2,
        name: "Oculta".into(),
        visible: false,
        locked: false,
    });
    d.active_layer = 2;
    d.add_object(Object::new(
        "Não exportar",
        Geometry::Rectangle(Rect::new(600., 400., 750., 550.)),
        Some([0, 0, 0, 255]),
        None,
        0.,
    ));
    d.add_page(false).unwrap();
    d.width = 400.;
    d.height = 300.;
    d.name = "Página2 - affine".into();
    let i = d.add_object(Object::new(
        "Elipse inclinada",
        Geometry::Ellipse(Rect::new(40., 40., 140., 140.)),
        Some([34, 197, 94, 255]),
        Some([10, 50, 10, 255]),
        3.,
    ));
    d.objects[i].transform = Affine::new([1., 0.2, 0.35, 1., 0., 0.]);
    d.add_object(Object::new(
        "Página2",
        Geometry::Text {
            position: Point::new(40., 180.),
            text: "Página 2".into(),
            size: 24.,
        },
        Some([30, 30, 30, 255]),
        None,
        0.,
    ));
    d.add_page(false).unwrap();
    d.width = 300.;
    d.height = 450.;
    d.name = "Página3 - vazia".into();
    d.activate_page(1);
    d
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{Geometry, Layer, Object};
    use vello::kurbo::{Affine, Rect};
    fn text(doc: &Document, all: bool) -> String {
        String::from_utf8_lossy(&encode(doc, all).unwrap()).into_owned()
    }
    #[test]
    fn multipage_dimensions_and_active_scope() {
        let mut d = Document::demo();
        d.add_page(false).unwrap();
        d.width = 400.;
        d.height = 300.;
        let old = d.clone();
        let pdf = text(&d, true);
        assert!(pdf.contains("/Count 2"));
        assert!(pdf.contains("/MediaBox [0 0 300 225]"));
        assert!(text(&d, false).contains("/Count 1"));
        assert_eq!(d, old);
    }
    #[test]
    fn xref_offsets_point_to_real_objects() {
        let bytes = encode(&Document::demo(), true).unwrap();
        let pdf = String::from_utf8_lossy(&bytes);
        let offset: usize = pdf
            .rsplit("startxref\n")
            .next()
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(&bytes[offset..offset + 5], b"xref\n");
        let lines: Vec<_> = std::str::from_utf8(&bytes[offset..])
            .unwrap()
            .lines()
            .collect();
        let count: usize = lines[1].split_whitespace().nth(1).unwrap().parse().unwrap();
        for id in 1..count {
            let position: usize = lines[id + 2][..10].parse().unwrap();
            assert!(bytes[position..].starts_with(format!("{id} 0 obj\n").as_bytes()));
        }
    }
    #[test]
    fn quadratic_conversion_is_exact() {
        let mut p = BezPath::new();
        p.move_to((0., 0.));
        p.quad_to((3., 6.), (9., 0.));
        let mut out = String::new();
        path(&mut out, &p).unwrap();
        assert!(out.contains("2 4 5 4 9 0 c"));
    }
    #[test]
    fn alpha_channels_and_affine_are_preserved() {
        let mut d = Document::demo();
        d.objects[0].fill = Some([255, 0, 0, 127]);
        d.objects[0].stroke = Some([0, 0, 255, 64]);
        d.objects[0].stroke_width = 3.;
        d.objects[0].transform = Affine::new([1., 0.2, 0.3, 1., 20., 30.]);
        let pdf = text(&d, true);
        assert!(pdf.contains("/ca 0.49803922 /CA 0.25098039"));
        assert!(pdf.contains("1 0.2 0.3 1 20 30 cm"));
    }
    #[test]
    fn hidden_objects_and_layers_are_not_painted() {
        let mut d = Document::default();
        d.add_object(Object::new(
            "visible",
            Geometry::Rectangle(Rect::new(0., 0., 20., 20.)),
            Some([0, 0, 255, 255]),
            None,
            0.,
        ));
        let mut hidden = d.objects[0].clone();
        hidden.fill = Some([255, 0, 0, 255]);
        hidden.visible = false;
        d.add_object(hidden);
        d.layers.push(Layer {
            id: 2,
            name: "hidden".into(),
            visible: false,
            locked: false,
        });
        d.active_layer = 2;
        d.add_object(Object::new(
            "hidden",
            Geometry::Rectangle(Rect::new(0., 0., 20., 20.)),
            Some([0, 255, 0, 255]),
            None,
            0.,
        ));
        let pdf = text(&d, true);
        assert!(pdf.contains("0 0 1 rg"));
        assert!(!pdf.contains("1 0 0 rg"));
        assert!(!pdf.contains("0 1 0 rg"));
    }
    #[test]
    fn evenodd_holes_and_text_are_vector_outlines() {
        let mut d = Document::demo();
        d.objects[0].evenodd = true;
        d.objects[0].stroke = Some([0, 0, 0, 255]);
        d.objects[0].stroke_width = 1.;
        d.name = "Ação".into();
        let pdf = text(&d, true);
        assert!(pdf.contains("B*\n"));
        assert!(pdf.matches(" c\n").count() > 20);
        assert!(!pdf.contains("/Font"));
        assert!(pdf.contains("<FEFF004100E700E3006F>"));
    }
    #[test]
    fn invalid_numbers_do_not_replace_previous_file() {
        let mut d = Document::demo();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/test-work/pdf");
        std::fs::create_dir_all(&root).unwrap();
        let file = root.join(format!(
            "atomic-{}.pdf",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        save(&file, &d, true).unwrap();
        let old = std::fs::read(&file).unwrap();
        d.objects[0].transform = Affine::translate((f64::NAN, 0.));
        assert!(save(&file, &d, true).is_err());
        assert_eq!(std::fs::read(&file).unwrap(), old);
        assert!(num(1e10).is_err());
        assert!(!num(1e-8).unwrap().contains('e'));
    }
}
