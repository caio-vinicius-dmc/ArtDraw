use crate::document::{Color, Document, Geometry, Layer, Object};
use std::fmt::Write;
use std::str::FromStr;
use vello::kurbo::{Affine, BezPath, Point, Rect};

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
fn color(c: Option<Color>) -> String {
    c.map_or_else(
        || "none".into(),
        |c| format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2]),
    )
}
pub fn export(doc: &Document) -> String {
    let mut s = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",doc.width,doc.height,doc.width,doc.height);
    s = s.replace(
        "<svg xmlns=",
        &format!(
            "<svg data-artdraw-version=\"1.9\" data-active-layer=\"{}\" data-next-id=\"{}\" xmlns=",
            doc.active_layer, doc.next_object_id
        ),
    );
    s = s.replace(
        " xmlns=",
        &format!(
            " data-page-id=\"{}\" data-page-name=\"{}\" data-next-page-id=\"{}\" xmlns=",
            doc.active_page,
            escape(&doc.name),
            doc.next_page_id
        ),
    );
    s.push_str("<metadata><artdraw-layers xmlns=\"urn:artdraw:document:1\">");
    for l in &doc.layers {
        let _ = write!(
            s,
            "<layer id=\"{}\" name=\"{}\" visible=\"{}\" locked=\"{}\"/>",
            l.id,
            escape(&l.name),
            l.visible,
            l.locked
        );
    }
    s.push_str("</artdraw-layers><artdraw-groups xmlns=\"urn:artdraw:document:1\">");
    for (child, parent) in &doc.group_parents {
        let _ = write!(s, "<group id=\"{child}\" parent=\"{parent}\"/>");
    }
    s.push_str("</artdraw-groups></metadata>\n");
    let mut open: Vec<u64> = Vec::new();
    for index in doc.paint_order() {
        let desired: Vec<_> = doc.objects[index]
            .group
            .map(|g| doc.group_chain(g).into_iter().rev().collect())
            .unwrap_or_default();
        let common = open
            .iter()
            .zip(&desired)
            .take_while(|(a, b)| a == b)
            .count();
        for _ in common..open.len() {
            s.push_str("</g>\n");
        }
        for group in desired.iter().skip(common) {
            let _ = writeln!(s, "<g data-artdraw-group=\"{group}\">");
        }
        open = desired;
        let o = &doc.objects[index];
        let a = o.transform.as_coeffs();
        let style=format!("fill=\"{}\" fill-opacity=\"{}\" stroke=\"{}\" stroke-opacity=\"{}\" stroke-width=\"{}\" stroke-linejoin=\"round\" stroke-linecap=\"round\" transform=\"matrix({} {} {} {} {} {})\" data-name=\"{}\"",color(o.fill),o.fill.map_or(1.,|c| f64::from(c[3])/255.),color(o.stroke),o.stroke.map_or(1.,|c| f64::from(c[3])/255.),o.stroke_width,a[0],a[1],a[2],a[3],a[4],a[5],escape(&o.name));
        let style = format!("{style} data-object-id=\"{}\"", o.id);
        let style = format!(
            "{style} fill-rule=\"{}\"",
            if o.evenodd { "evenodd" } else { "nonzero" }
        );
        let style=format!("{style} data-artdraw-object=\"1\" data-index=\"{index}\" data-layer=\"{}\" data-group=\"{}\" data-visible=\"{}\" data-locked=\"{}\" {}",o.layer,o.group.unwrap_or(0),o.visible,o.locked,if doc.visible(index){""}else{"display=\"none\""});
        let geometry_data = match &o.geometry {
            Geometry::Rectangle(r) | Geometry::Ellipse(r) => {
                format!("data-artdraw-rect=\"{} {} {} {}\"", r.x0, r.y0, r.x1, r.y1)
            }
            Geometry::Path { nodes, closed } => format!(
                "data-artdraw-closed=\"{closed}\" data-artdraw-nodes=\"{}\"",
                nodes
                    .iter()
                    .map(|n| format!(
                        "{} {} {} {} {} {}",
                        n.p.x, n.p.y, n.incoming.x, n.incoming.y, n.outgoing.x, n.outgoing.y
                    ))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            _ => String::new(),
        };
        let style = format!(
            "{style} {geometry_data} data-artdraw-transform=\"{} {} {} {} {} {}\"",
            a[0], a[1], a[2], a[3], a[4], a[5]
        );
        match &o.geometry {
            Geometry::Rectangle(r) => {
                let _ = writeln!(
                    s,
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" {style}/>",
                    r.x0,
                    r.y0,
                    r.width(),
                    r.height()
                );
            }
            Geometry::Ellipse(r) => {
                let _ = writeln!(
                    s,
                    "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" {style}/>",
                    r.center().x,
                    r.center().y,
                    r.width() / 2.,
                    r.height() / 2.
                );
            }
            Geometry::Text {
                position,
                text,
                size,
            } => {
                let _=writeln!(s,"<g {style} data-artdraw-text=\"{}\" data-x=\"{}\" data-y=\"{}\" data-size=\"{}\"><path d=\"{}\"/></g>",escape(&text.replace('\n',"&#10;" )).replace("&amp;#10;","&#10;"),position.x,position.y,size,o.local_path().to_svg());
            }
            _ => {
                let _ = writeln!(s, "<path d=\"{}\" {style}/>", o.local_path().to_svg());
            }
        }
    }
    for _ in open {
        s.push_str("</g>\n");
    }
    s.push_str("</svg>\n");
    s
}
#[derive(Clone)]
struct Style {
    fill: Option<Color>,
    stroke: Option<Color>,
    width: f64,
    opacity: f64,
    font_size: f64,
    fill_opacity: f64,
    stroke_opacity: f64,
    evenodd: bool,
    group: Option<u64>,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            fill: Some([0, 0, 0, 255]),
            stroke: None,
            width: 1.,
            opacity: 1.,
            font_size: 24.,
            fill_opacity: 1.,
            stroke_opacity: 1.,
            evenodd: false,
            group: None,
        }
    }
}
fn property<'a>(node: roxmltree::Node<'a, 'a>, name: &str) -> Option<&'a str> {
    node.attribute("style")
        .and_then(|style| {
            style
                .split(';')
                .filter_map(|p| p.split_once(':'))
                .filter(|(k, _)| k.trim() == name)
                .map(|(_, v)| v.trim())
                .next_back()
        })
        .or_else(|| node.attribute(name))
}
fn number(value: &str) -> Option<f64> {
    let v = value.trim();
    let (text, factor) = if let Some(t) = v.strip_suffix("mm") {
        (t, 96. / 25.4)
    } else if let Some(t) = v.strip_suffix("cm") {
        (t, 96. / 2.54)
    } else if let Some(t) = v.strip_suffix("in") {
        (t, 96.)
    } else if let Some(t) = v.strip_suffix("pt") {
        (t, 96. / 72.)
    } else if let Some(t) = v.strip_suffix("px") {
        (t, 1.)
    } else {
        (v, 1.)
    };
    text.parse::<f64>()
        .ok()
        .filter(|x| x.is_finite())
        .map(|x| x * factor)
}
fn attr(node: roxmltree::Node<'_, '_>, name: &str, default: f64) -> f64 {
    node.attribute(name).and_then(number).unwrap_or(default)
}
fn paint(v: &str, warnings: &mut Vec<String>) -> Option<Color> {
    if v == "none" {
        return None;
    }
    match svgtypes::Color::from_str(v) {
        Ok(c) => Some([c.red, c.green, c.blue, c.alpha]),
        Err(_) => {
            warnings.push(format!("Pintura não suportada: {v}; usada cor cinza"));
            Some([110, 110, 110, 255])
        }
    }
}
pub fn import(xml: &str) -> Result<(Document, Vec<String>), String> {
    if xml.len() > 16 * 1024 * 1024 {
        return Err("SVG excede o limite de 16 MiB".into());
    }
    let tree = roxmltree::Document::parse(xml).map_err(|e| format!("SVG/XML inválido: {e}"))?;
    let root = tree.root_element();
    if root.tag_name().name() != "svg" {
        return Err("O elemento raiz deve ser <svg>".into());
    }
    let vb: Vec<f64> = root
        .attribute("viewBox")
        .unwrap_or("")
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .filter_map(number)
        .collect();
    let (vw, vh) = if vb.len() == 4 {
        (vb[2], vb[3])
    } else {
        (794., 1123.)
    };
    let width = attr(root, "width", vw).clamp(1., 100000.);
    let height = attr(root, "height", vh).clamp(1., 100000.);
    let mut transform = Affine::IDENTITY;
    if vb.len() == 4 && vw > 0. && vh > 0. {
        let sx = width / vw;
        let sy = height / vh;
        if root.attribute("preserveAspectRatio") == Some("none") {
            transform = Affine::scale_non_uniform(sx, sy) * Affine::translate((-vb[0], -vb[1]));
        } else {
            let scale = sx.min(sy);
            transform = Affine::translate(((width - vw * scale) / 2., (height - vh * scale) / 2.))
                * Affine::scale(scale)
                * Affine::translate((-vb[0], -vb[1]));
        }
    }
    let mut doc = Document::with_size(width, height);
    let native = root.attribute("data-artdraw-version").is_some();
    if native {
        let id = root
            .attribute("data-page-id")
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|&v| v > 0 && v < u64::MAX)
            .unwrap_or(1);
        doc.pages[0].id = id;
        doc.active_page = id;
        doc.next_page_id = root
            .attribute("data-next-page-id")
            .and_then(|v| v.parse().ok())
            .filter(|&v| v > id)
            .unwrap_or(id + 1);
        if let Some(name) = root
            .attribute("data-page-name")
            .filter(|n| !n.trim().is_empty() && n.chars().count() <= 256)
        {
            doc.pages[0].name = name.into();
        }
        let layers: Vec<_> = root
            .descendants()
            .filter(|n| {
                n.has_tag_name("layer") && n.ancestors().any(|p| p.has_tag_name("artdraw-layers"))
            })
            .filter_map(|n| {
                Some(Layer {
                    id: n.attribute("id")?.parse().ok()?,
                    name: n.attribute("name").unwrap_or("Camada").into(),
                    visible: n.attribute("visible") != Some("false"),
                    locked: n.attribute("locked") == Some("true"),
                })
            })
            .collect();
        if !layers.is_empty() {
            let mut ids = std::collections::BTreeSet::new();
            if layers.iter().any(|l| !ids.insert(l.id) || l.id == 0) {
                return Err("IDs de camada inválidos".into());
            }
            doc.layers = layers;
            doc.active_layer = root
                .attribute("data-active-layer")
                .and_then(|v| v.parse().ok())
                .filter(|id| doc.layers.iter().any(|l| l.id == *id))
                .unwrap_or(doc.layers[0].id);
        }
    }
    if native {
        for g in root.descendants().filter(|n| {
            n.has_tag_name("group") && n.parent().is_some_and(|p| p.has_tag_name("artdraw-groups"))
        }) {
            let child = g
                .attribute("id")
                .and_then(|v| v.parse().ok())
                .ok_or("ID de grupo SVG inválido")?;
            let parent = g
                .attribute("parent")
                .and_then(|v| v.parse().ok())
                .ok_or("Pai de grupo SVG inválido")?;
            if doc.group_parents.insert(child, parent).is_some() {
                return Err("Grupo SVG repetido".into());
            }
        }
    }
    let mut warnings = Vec::new();
    walk(root, &Style::default(), transform, &mut doc, &mut warnings)?;
    if native {
        let indices: Vec<_> = root
            .descendants()
            .filter(|n| n.attribute("data-artdraw-object") == Some("1"))
            .map(|n| {
                n.attribute("data-index")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(usize::MAX)
            })
            .collect();
        if indices.len() == doc.objects.len() {
            let mut ordered: Vec<_> = indices
                .into_iter()
                .zip(std::mem::take(&mut doc.objects))
                .collect();
            ordered.sort_by_key(|(i, _)| *i);
            doc.objects = ordered.into_iter().map(|(_, o)| o).collect();
        }
    }
    doc.next_object_id = root
        .attribute("data-next-id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    doc.ensure_object_ids();
    if !native {
        doc.prune_groups();
    }
    doc.validate_groups()?;
    warnings.sort();
    warnings.dedup();
    Ok((doc, warnings))
}
fn walk(
    node: roxmltree::Node<'_, '_>,
    inherited: &Style,
    parent: Affine,
    doc: &mut Document,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    if !node.is_element() {
        return Ok(());
    }
    if node.attribute("data-artdraw-object") != Some("1")
        && (property(node, "display") == Some("none")
            || property(node, "visibility") == Some("hidden"))
    {
        return Ok(());
    }
    let tag = node.tag_name().name();
    if matches!(
        tag,
        "defs" | "metadata" | "title" | "desc" | "style" | "clipPath"
    ) {
        if tag == "style" {
            warnings.push("CSS em <style> não é aplicado; use estilos inline".into());
        }
        return Ok(());
    }
    let mut st = inherited.clone();
    let native_wrapper = node.attribute("data-artdraw-group").is_some()
        && node
            .ancestors()
            .any(|n| n.has_tag_name("svg") && n.attribute("data-artdraw-version").is_some());
    if tag == "g"
        && !native_wrapper
        && node.attribute("data-artdraw-text").is_none()
        && node.attribute("data-artdraw-object").is_none()
    {
        // XML offsets uniquely reserve even groups with no direct leaf before recursion.
        let group = node.range().start as u64 + 1;
        if let Some(parent) = st.group {
            doc.group_parents.insert(group, parent);
        }
        st.group = Some(group);
    }
    if let Some(v) = property(node, "fill") {
        st.fill = paint(v, warnings);
    }
    if let Some(v) = property(node, "stroke") {
        st.stroke = paint(v, warnings);
    }
    if let Some(v) = property(node, "stroke-width").and_then(number) {
        st.width = v.max(0.);
    }
    if let Some(v) = property(node, "font-size").and_then(number) {
        st.font_size = v.clamp(1., 2000.);
    }
    if let Some(v) = property(node, "fill-opacity").and_then(number) {
        st.fill_opacity = v.clamp(0., 1.);
    }
    if let Some(v) = property(node, "stroke-opacity").and_then(number) {
        st.stroke_opacity = v.clamp(0., 1.);
    }
    if let Some(v) = property(node, "fill-rule") {
        st.evenodd = v == "evenodd";
    }
    st.opacity *= property(node, "opacity")
        .and_then(number)
        .unwrap_or(1.)
        .clamp(0., 1.);
    let alpha = |c: Option<Color>, opacity: f64| {
        c.map(|mut c| {
            c[3] = (f64::from(c[3]) * st.opacity * opacity).round() as u8;
            c
        })
    };
    let fill = alpha(st.fill, st.fill_opacity);
    let stroke = alpha(st.stroke, st.stroke_opacity);
    let local = if let Some(v) = node.attribute("transform") {
        let t =
            svgtypes::Transform::from_str(v).map_err(|e| format!("Transformação inválida: {e}"))?;
        Affine::new([t.a, t.b, t.c, t.d, t.e, t.f])
    } else {
        Affine::IDENTITY
    };
    let transform = parent * local;
    if node.attribute("clip-path").is_some()
        || node.attribute("filter").is_some()
        || node.attribute("mask").is_some()
    {
        warnings.push("Recortes, filtros e máscaras não são aplicados".into());
    }
    let geometry = if let Some(text) = node.attribute("data-artdraw-text") {
        Some(Geometry::Text {
            position: Point::new(attr(node, "data-x", 0.), attr(node, "data-y", 0.)),
            text: text.into(),
            size: attr(node, "data-size", 24.),
        })
    } else {
        match tag {
            "svg" | "g" | "a" => None,
            "rect" => {
                let x = attr(node, "x", 0.);
                let y = attr(node, "y", 0.);
                let w = attr(node, "width", 0.);
                let h = attr(node, "height", 0.);
                if node.attribute("rx").is_some() {
                    warnings.push("Retângulo arredondado importado sem arredondamento".into());
                }
                Some(Geometry::Rectangle(Rect::new(
                    x,
                    y,
                    x + w.max(0.),
                    y + h.max(0.),
                )))
            }
            "ellipse" | "circle" => {
                let x = attr(node, "cx", 0.);
                let y = attr(node, "cy", 0.);
                let rx = attr(node, "rx", attr(node, "r", 0.)).abs();
                let ry = attr(node, "ry", attr(node, "r", 0.)).abs();
                Some(Geometry::Ellipse(Rect::new(x - rx, y - ry, x + rx, y + ry)))
            }
            "path" => {
                let path = BezPath::from_svg(node.attribute("d").unwrap_or(""))
                    .map_err(|e| format!("Caminho SVG inválido: {e}"))?;
                let parts = crate::document::objects_from_path(&path, fill, stroke, st.width);
                if parts.len() == 1 {
                    Some(parts[0].geometry.clone())
                } else {
                    Some(Geometry::Compound(path))
                }
            }
            "line" => {
                let mut path = BezPath::new();
                path.move_to((attr(node, "x1", 0.), attr(node, "y1", 0.)));
                path.line_to((attr(node, "x2", 0.), attr(node, "y2", 0.)));
                Some(Geometry::Compound(path))
            }
            "polygon" | "polyline" => {
                let v: Vec<_> =
                    svgtypes::NumberListParser::from(node.attribute("points").unwrap_or(""))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| format!("Pontos SVG inválidos: {e}"))?;
                let mut path = BezPath::new();
                for (i, p) in v.chunks_exact(2).enumerate() {
                    if i == 0 {
                        path.move_to((p[0], p[1]));
                    } else {
                        path.line_to((p[0], p[1]));
                    }
                }
                if tag == "polygon" {
                    path.close_path();
                }
                Some(Geometry::Compound(path))
            }
            "text" => {
                let text = node
                    .descendants()
                    .filter(|n| n.is_text())
                    .filter_map(|n| n.text())
                    .collect::<String>();
                warnings.push("Texto SVG externo usa a fonte Ubuntu Light incorporada; layout avançado não é preservado".into());
                Some(Geometry::Text {
                    position: Point::new(attr(node, "x", 0.), attr(node, "y", 0.) - st.font_size),
                    text,
                    size: st.font_size,
                })
            }
            other => {
                warnings.push(format!("Elemento não suportado ignorado: <{other}>"));
                None
            }
        }
    };
    if let Some(geometry) = geometry {
        let mut object = Object::new(
            node.attribute("data-name")
                .or_else(|| node.attribute("id"))
                .unwrap_or(tag),
            geometry,
            fill,
            stroke,
            st.width,
        );
        object.transform = transform;
        if node.attribute("data-artdraw-object") == Some("1") {
            let parse = |name: &str| -> Result<Option<Vec<f64>>, String> {
                node.attribute(name)
                    .map(|s| {
                        s.split_whitespace()
                            .map(|part| {
                                part.parse::<f64>()
                                    .ok()
                                    .filter(|x| x.is_finite())
                                    .ok_or_else(|| format!("Metadado numérico inválido: {name}"))
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .transpose()
            };
            if let Some(v) = parse("data-artdraw-transform")? {
                if v.len() != 6 {
                    return Err("Matriz nativa inválida".into());
                }
                object.transform = Affine::new(v.try_into().unwrap());
            }
            if let Some(v) = parse("data-artdraw-rect")? {
                if v.len() != 4 {
                    return Err("Retângulo nativo inválido".into());
                }
                let r = Rect::new(v[0], v[1], v[2], v[3]);
                object.geometry = if tag == "ellipse" {
                    Geometry::Ellipse(r)
                } else {
                    Geometry::Rectangle(r)
                };
            }
            if let Some(v) = parse("data-artdraw-nodes")? {
                if v.len() % 6 != 0 {
                    return Err("Nós nativos inválidos".into());
                }
                object.geometry = Geometry::Path {
                    nodes: v
                        .chunks_exact(6)
                        .map(|c| crate::document::Node {
                            p: Point::new(c[0], c[1]),
                            incoming: Point::new(c[2], c[3]),
                            outgoing: Point::new(c[4], c[5]),
                        })
                        .collect(),
                    closed: node.attribute("data-artdraw-closed") == Some("true"),
                };
            }
        }
        object.id = node
            .attribute("data-object-id")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        object.evenodd = st.evenodd;
        object.layer = node
            .attribute("data-layer")
            .and_then(|s| s.parse().ok())
            .filter(|id| doc.layers.iter().any(|l| l.id == *id))
            .unwrap_or(doc.active_layer);
        object.group = if node.attribute("data-artdraw-object").is_some() {
            node.attribute("data-group")
                .and_then(|s| s.parse::<u64>().ok())
                .filter(|&g| g != 0)
        } else {
            st.group
        };
        object.visible = node.attribute("data-visible") != Some("false");
        object.locked = node.attribute("data-locked") == Some("true");
        if !object.path().elements().iter().all(|el| match el {
            vello::kurbo::PathEl::MoveTo(p) | vello::kurbo::PathEl::LineTo(p) => p.is_finite(),
            vello::kurbo::PathEl::QuadTo(a, b) => a.is_finite() && b.is_finite(),
            vello::kurbo::PathEl::CurveTo(a, b, c) => {
                a.is_finite() && b.is_finite() && c.is_finite()
            }
            _ => true,
        }) {
            return Err("Geometria contém coordenadas inválidas".into());
        }
        if doc.objects.len() >= 50000 {
            return Err("Limite de 50 mil objetos excedido".into());
        }
        doc.objects.push(object);
        return Ok(());
    }
    for child in node.children() {
        walk(child, &st, transform, doc, warnings)?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn own_svg_roundtrip_keeps_text_and_cubics() {
        let d = Document::demo();
        let (r, w) = import(&export(&d)).unwrap();
        assert!(w.is_empty());
        assert_eq!(r.objects.len(), d.objects.len());
        for (a, b) in d.objects.iter().zip(&r.objects) {
            assert!((a.bounds().width() - b.bounds().width()).abs() < 0.01);
            assert_eq!(a.geometry, b.geometry);
        }
    }
    #[test]
    fn group_transform_and_inherited_color() {
        let (d,_)=import(r##"<svg width="100" height="100"><g fill="#ff0000" transform="translate(10,20)"><rect width="30" height="40"/></g></svg>"##).unwrap();
        assert_eq!(d.objects[0].bounds(), Rect::new(10., 20., 40., 60.));
        assert_eq!(d.objects[0].fill, Some([255, 0, 0, 255]));
    }
    #[test]
    fn arcs_and_quadratic_import() {
        let (d, _) =
            import(r#"<svg><path d="M0 0 Q10 20 20 0 A10 10 0 0 1 40 0"/></svg>"#).unwrap();
        assert!(d.objects[0].local_path().elements().len() > 2);
    }
    #[test]
    fn malformed_svg_is_error() {
        assert!(import("<svg><path></svg>").is_err());
    }
    #[test]
    fn xml_text_escaping_roundtrip() {
        let mut d = Document::default();
        d.objects.push(Object::new(
            "A&B",
            Geometry::Text {
                position: Point::ZERO,
                text: "A&B <oi>\nOlá".into(),
                size: 24.,
            },
            Some([0, 0, 0, 255]),
            None,
            0.,
        ));
        let (r, _) = import(&export(&d)).unwrap();
        assert_eq!(d.objects[0].geometry, r.objects[0].geometry);
    }
    #[test]
    fn physical_units_and_viewbox() {
        let (d,_)=import(r#"<svg width="25.4mm" height="25.4mm" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>"#).unwrap();
        assert!((d.width - 96.).abs() < 1e-6);
        assert!((d.objects[0].bounds().width() - 96.).abs() < 1e-6);
    }
    #[test]
    fn evenodd_holes_and_inherited_opacity() {
        let (d,_)=import(r##"<svg><g fill="#ff0000" fill-opacity="0.5" opacity="0.5"><path fill-rule="evenodd" d="M0 0H100V100H0Z M25 25H75V75H25Z"/></g></svg>"##).unwrap();
        assert_eq!(d.objects[0].fill, Some([255, 0, 0, 64]));
        assert!(d.objects[0].evenodd);
        assert!(!d.objects[0].hit(Point::new(50., 50.), 1.));
        assert!(d.objects[0].hit(Point::new(10., 10.), 1.));
        let (again, _) = import(&export(&d)).unwrap();
        assert!(again.objects[0].evenodd);
        assert_eq!(again.objects[0].fill, d.objects[0].fill);
    }

    #[test]
    fn native_svg_preserves_layers_groups_and_hidden_objects() {
        let mut d = Document::demo();
        d.layers.push(Layer {
            id: 2,
            name: "Oculta & segura".into(),
            visible: false,
            locked: true,
        });
        d.active_layer = 2;
        d.objects[0].layer = 2;
        d.objects[0].group = Some(7);
        d.objects[1].group = Some(7);
        d.objects[2].visible = false;
        d.objects[3].locked = true;
        let (r, w) = import(&export(&d)).unwrap();
        assert!(w.is_empty());
        assert_eq!(d, r);
    }
}

#[cfg(test)]
mod page_tests {
    use super::*;
    #[test]
    fn single_page_svg_preserves_name_identity_and_allocator_after_deletions() {
        let mut d = Document::demo();
        let second = d.add_page(true).unwrap();
        d.delete_page(1).unwrap();
        assert_eq!(d.active_page, second);
        d.name = "Página ação & detalhe".into();
        let (loaded, warnings) = import(&export(&d)).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(loaded, d);
    }
    #[test]
    fn nested_svg_groups_import_and_artdraw_roundtrip() {
        let (d,warnings)=import(r#"<svg width="300" height="300"><g transform="translate(10 20)"><g><rect width="10" height="20"/><circle cx="30" cy="40" r="5"/></g><rect x="80" width="10" height="20"/></g></svg>"#).unwrap();
        assert!(warnings.is_empty());
        assert_eq!(d.objects.len(), 3);
        assert_eq!(d.group_parents.len(), 1);
        assert_eq!(d.root_group(0), d.root_group(2));
        assert_ne!(d.objects[0].group, d.objects[2].group);
        assert_eq!(d.objects[0].bounds().x0, 10.);
        assert_eq!(d.objects[0].bounds().y0, 20.);
        let exported = export(&d);
        let tree = roxmltree::Document::parse(&exported).unwrap();
        let wrappers: Vec<_> = tree
            .descendants()
            .filter(|n| n.attribute("data-artdraw-group").is_some())
            .collect();
        assert_eq!(wrappers.len(), 2);
        assert!(wrappers[1].ancestors().skip(1).any(|n| n == wrappers[0]));
        assert_eq!(import(&exported).unwrap().0, d);
        let malformed = export(&d).replace(
            "</artdraw-groups>",
            "<group id=\"999\" parent=\"998\"/></artdraw-groups>",
        );
        assert!(import(&malformed).is_err());
    }
}
