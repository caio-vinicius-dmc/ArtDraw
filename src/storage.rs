//! Versioned native document schema and atomic saves. Does not rely on SVG metadata.
use crate::document::{Document, Geometry, Layer, Node, Object, Page};
use std::{
    fmt::Write,
    fs,
    io::Write as IoWrite,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
use vello::kurbo::{Affine, BezPath, Point, Rect};
const LIMIT: usize = 32 * 1024 * 1024;
static SERIAL: AtomicU64 = AtomicU64::new(1);
fn escaped(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\n', "&#10;")
        .replace('\r', "&#13;")
        .replace('\t', "&#9;")
}
fn color(c: Option<[u8; 4]>) -> String {
    c.map_or_else(
        || "none".into(),
        |c| format!("{} {} {} {}", c[0], c[1], c[2], c[3]),
    )
}
fn numbers(v: impl IntoIterator<Item = f64>) -> String {
    v.into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
pub fn encode(doc: &Document) -> String {
    let mut out = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<artdraw version=\"3\" active-page=\"{}\" next-page-id=\"{}\" next-id=\"{}\">\n<pages>\n", doc.active_page, doc.next_page_id, doc.next_object_id);
    for page in &doc.pages {
        encode_page(page, &mut out);
    }
    out.push_str("</pages>\n</artdraw>\n");
    out
}
fn encode_page(doc: &Page, out: &mut String) {
    let _ = writeln!(
        out,
        "<page id=\"{}\" name=\"{}\" width=\"{}\" height=\"{}\" active-layer=\"{}\">\n<layers>",
        doc.id,
        escaped(&doc.name),
        doc.width,
        doc.height,
        doc.active_layer
    );
    for l in &doc.layers {
        let _ = writeln!(
            out,
            "<layer id=\"{}\" name=\"{}\" visible=\"{}\" locked=\"{}\"/>",
            l.id,
            escaped(&l.name),
            l.visible,
            l.locked
        );
    }
    out.push_str("</layers>\n<groups>\n");
    for (child, parent) in &doc.group_parents {
        let _ = writeln!(out, "<group id=\"{child}\" parent=\"{parent}\"/>");
    }
    out.push_str("</groups>\n<objects>\n");
    for o in &doc.objects {
        let _=writeln!(out,"<object id=\"{}\" name=\"{}\" layer=\"{}\" group=\"{}\" visible=\"{}\" locked=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" evenodd=\"{}\" transform=\"{}\">",o.id,escaped(&o.name),o.layer,o.group.unwrap_or(0),o.visible,o.locked,color(o.fill),color(o.stroke),o.stroke_width,o.evenodd,numbers(o.transform.as_coeffs()));
        match &o.geometry {
            Geometry::Rectangle(r) | Geometry::Ellipse(r) => {
                let _ = writeln!(
                    out,
                    "<{} rect=\"{}\"/>",
                    if matches!(o.geometry, Geometry::Rectangle(_)) {
                        "rectangle"
                    } else {
                        "ellipse"
                    },
                    numbers([r.x0, r.y0, r.x1, r.y1])
                );
            }
            Geometry::Path { nodes, closed } => {
                let _ = writeln!(out, "<path closed=\"{closed}\">");
                for n in nodes {
                    let _ = writeln!(
                        out,
                        "<node p=\"{}\" in=\"{}\" out=\"{}\"/>",
                        numbers([n.p.x, n.p.y]),
                        numbers([n.incoming.x, n.incoming.y]),
                        numbers([n.outgoing.x, n.outgoing.y])
                    );
                }
                out.push_str("</path>\n");
            }
            Geometry::Text {
                position,
                text,
                size,
            } => {
                let _ = writeln!(
                    out,
                    "<text position=\"{}\" size=\"{}\" value=\"{}\"/>",
                    numbers([position.x, position.y]),
                    size,
                    escaped(text)
                );
            }
            Geometry::Compound(path) => {
                let _ = writeln!(out, "<compound d=\"{}\"/>", escaped(&path.to_svg()));
            }
        }
        out.push_str("</object>\n");
    }
    out.push_str("</objects>\n</page>\n");
}
fn attr<'a>(node: roxmltree::Node<'a, '_>, key: &str) -> Result<&'a str, String> {
    node.attribute(key)
        .ok_or_else(|| format!("Atributo nativo ausente: {key}"))
}
fn number(node: roxmltree::Node<'_, '_>, key: &str) -> Result<f64, String> {
    let v = attr(node, key)?
        .parse::<f64>()
        .map_err(|_| format!("Número inválido: {key}"))?;
    if !v.is_finite() {
        return Err(format!("Número não finito: {key}"));
    }
    Ok(v)
}
fn integer(node: roxmltree::Node<'_, '_>, key: &str) -> Result<u64, String> {
    attr(node, key)?
        .parse()
        .map_err(|_| format!("ID inválido: {key}"))
}
fn boolean(node: roxmltree::Node<'_, '_>, key: &str) -> Result<bool, String> {
    match attr(node, key)? {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("Booleano inválido: {key}")),
    }
}
fn array<const N: usize>(node: roxmltree::Node<'_, '_>, key: &str) -> Result<[f64; N], String> {
    let v = attr(node, key)?
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| format!("Coordenada inválida: {key}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if v.len() != N || v.iter().any(|v| !v.is_finite()) {
        return Err(format!("Coordenadas inválidas: {key}"));
    }
    Ok(v.try_into().unwrap())
}
fn point(node: roxmltree::Node<'_, '_>, key: &str) -> Result<Point, String> {
    let a = array::<2>(node, key)?;
    Ok(Point::new(a[0], a[1]))
}
fn parse_color(node: roxmltree::Node<'_, '_>, key: &str) -> Result<Option<[u8; 4]>, String> {
    let s = attr(node, key)?;
    if s == "none" {
        return Ok(None);
    }
    let v = s
        .split_whitespace()
        .map(|v| v.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| format!("Cor inválida: {key}"))?;
    Ok(Some(
        v.try_into().map_err(|_| format!("Cor incompleta: {key}"))?,
    ))
}
pub fn decode(xml: &str) -> Result<Document, String> {
    if xml.len() > LIMIT {
        return Err("Documento excede32MiB".into());
    }
    let parsed =
        roxmltree::Document::parse(xml).map_err(|e| format!("Documento nativo inválido: {e}"))?;
    let root = parsed.root_element();
    if root.tag_name().name() != "artdraw" {
        return Err("Este arquivo não é um documento ArtDraw".into());
    }
    let version = attr(root, "version")?;
    if !matches!(version, "1" | "2" | "3") {
        return Err("Versão de documento ArtDraw não suportada; atualize o aplicativo".into());
    }
    let next_object_id = integer(root, "next-id")?;
    if version == "1" {
        let page = decode_page(root, next_object_id)?;
        return Ok(Document {
            pages: vec![page],
            active_page: 1,
            next_page_id: 2,
            next_object_id,
        });
    }
    let mut doc = Document {
        pages: Vec::new(),
        active_page: integer(root, "active-page")?,
        next_page_id: integer(root, "next-page-id")?,
        next_object_id,
    };
    let pages = root
        .children()
        .find(|n| n.has_tag_name("pages"))
        .ok_or("Páginas ausentes")?;
    let mut page_ids = std::collections::BTreeSet::new();
    let mut object_ids = std::collections::BTreeSet::new();
    for n in pages.children().filter(|n| n.is_element()) {
        if !n.has_tag_name("page") || doc.pages.len() >= 1000 {
            return Err("Páginas inválidas/excessivas".into());
        }
        let mut page = decode_page(n, next_object_id)?;
        page.id = integer(n, "id")?;
        page.name = attr(n, "name")?.into();
        if page.id == 0
            || !page_ids.insert(page.id)
            || page.name.trim().is_empty()
            || page.name.chars().count() > 256
        {
            return Err("Identidade/nome de página inválido".into());
        }
        for o in &page.objects {
            if !object_ids.insert(o.id) || object_ids.len() > 50000 {
                return Err("ID de objeto repetido entre páginas ou limite global excedido".into());
            }
        }
        doc.pages.push(page);
    }
    if !page_ids.contains(&doc.active_page) || page_ids.iter().any(|&id| id >= doc.next_page_id) {
        return Err("Página ativa/contador inválido".into());
    }
    Ok(doc)
}
fn decode_page(root: roxmltree::Node<'_, '_>, next_object_id: u64) -> Result<Page, String> {
    let width = number(root, "width")?;
    let height = number(root, "height")?;
    if !(1.0..=100000.0).contains(&width) || !(1.0..=100000.0).contains(&height) {
        return Err("Dimensões de página inválidas".into());
    }
    let mut doc = Page {
        id: 1,
        name: "Página 1".into(),
        width,
        height,
        objects: Vec::new(),
        layers: Vec::new(),
        active_layer: integer(root, "active-layer")?,
        group_parents: Default::default(),
    };
    let layers = root
        .children()
        .find(|n| n.has_tag_name("layers"))
        .ok_or("Camadas ausentes")?;
    let mut layer_ids = std::collections::BTreeSet::new();
    for n in layers.children().filter(|n| n.is_element()) {
        if !n.has_tag_name("layer") {
            return Err("Elemento de camada desconhecido".into());
        }
        let id = integer(n, "id")?;
        if id == 0 || !layer_ids.insert(id) {
            return Err("ID de camada repetido/inválido".into());
        }
        doc.layers.push(Layer {
            id,
            name: attr(n, "name")?.into(),
            visible: boolean(n, "visible")?,
            locked: boolean(n, "locked")?,
        });
    }
    if !layer_ids.contains(&doc.active_layer) {
        return Err("Camada ativa ausente".into());
    }
    let objects = root
        .children()
        .find(|n| n.has_tag_name("objects"))
        .ok_or("Objetos ausentes")?;
    let mut ids = std::collections::BTreeSet::new();
    for n in objects.children().filter(|n| n.is_element()) {
        if doc.objects.len() >= 50000 {
            return Err("Limite de50mil objetos excedido".into());
        }
        if !n.has_tag_name("object") {
            return Err("Elemento de objeto desconhecido".into());
        }
        let children: Vec<_> = n.children().filter(|n| n.is_element()).collect();
        if children.len() != 1 {
            return Err("Objeto deve ter uma geometria".into());
        }
        let g = children[0];
        let geometry = match g.tag_name().name() {
            "rectangle" | "ellipse" => {
                let r = array::<4>(g, "rect")?;
                if r[2] < r[0] || r[3] < r[1] {
                    return Err("Retângulo inválido".into());
                }
                let r = Rect::new(r[0], r[1], r[2], r[3]);
                if g.has_tag_name("rectangle") {
                    Geometry::Rectangle(r)
                } else {
                    Geometry::Ellipse(r)
                }
            }
            "path" => {
                let mut nodes = Vec::new();
                for p in g.children().filter(|n| n.is_element()) {
                    if !p.has_tag_name("node") || nodes.len() >= 1000000 {
                        return Err("Nós inválidos/excessivos".into());
                    }
                    nodes.push(Node {
                        p: point(p, "p")?,
                        incoming: point(p, "in")?,
                        outgoing: point(p, "out")?,
                    });
                }
                Geometry::Path {
                    nodes,
                    closed: boolean(g, "closed")?,
                }
            }
            "text" => {
                let size = number(g, "size")?;
                if !(1.0..=4096.0).contains(&size) {
                    return Err("Tamanho de texto inválido".into());
                }
                Geometry::Text {
                    position: point(g, "position")?,
                    size,
                    text: attr(g, "value")?.into(),
                }
            }
            "compound" => Geometry::Compound(
                BezPath::from_svg(attr(g, "d")?)
                    .map_err(|e| format!("Caminho composto inválido: {e}"))?,
            ),
            _ => return Err("Geometria nativa desconhecida".into()),
        };
        let id = integer(n, "id")?;
        if id == 0 || !ids.insert(id) {
            return Err("ID de objeto repetido/inválido".into());
        }
        let layer = integer(n, "layer")?;
        if !layer_ids.contains(&layer) {
            return Err("Camada do objeto ausente".into());
        }
        let stroke_width = number(n, "stroke-width")?;
        if stroke_width < 0. {
            return Err("Espessura de contorno negativa".into());
        }
        let mut o = Object::new(
            attr(n, "name")?,
            geometry,
            parse_color(n, "fill")?,
            parse_color(n, "stroke")?,
            stroke_width,
        );
        o.id = id;
        o.layer = layer;
        o.group = Some(integer(n, "group")?).filter(|&g| g != 0);
        o.visible = boolean(n, "visible")?;
        o.locked = boolean(n, "locked")?;
        o.evenodd = boolean(n, "evenodd")?;
        o.transform = Affine::new(array::<6>(n, "transform")?);
        if !o.path().elements().iter().all(|e| match e {
            vello::kurbo::PathEl::MoveTo(p) | vello::kurbo::PathEl::LineTo(p) => p.is_finite(),
            vello::kurbo::PathEl::QuadTo(a, b) => a.is_finite() && b.is_finite(),
            vello::kurbo::PathEl::CurveTo(a, b, c) => {
                a.is_finite() && b.is_finite() && c.is_finite()
            }
            _ => true,
        }) {
            return Err("Geometria contém valores inválidos".into());
        }
        doc.objects.push(o);
    }
    if let Some(groups) = root.children().find(|n| n.has_tag_name("groups")) {
        for g in groups.children().filter(|n| n.is_element()) {
            if !g.has_tag_name("group") || doc.group_parents.len() >= 50000 {
                return Err("Grupos inválidos/excessivos".into());
            }
            if doc
                .group_parents
                .insert(integer(g, "id")?, integer(g, "parent")?)
                .is_some()
            {
                return Err("Grupo repetido".into());
            }
        }
    }
    doc.validate_groups()?;
    if next_object_id == 0 || ids.iter().any(|&id| id >= next_object_id) {
        return Err("Contador de IDs inválido".into());
    }
    Ok(doc)
}
pub fn read(path: &Path) -> Result<(Document, Vec<String>), String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() > LIMIT as u64 {
        return Err("Arquivo excede32MiB".into());
    }
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    if path
        .extension()
        .is_some_and(|v| v.eq_ignore_ascii_case("artdraw"))
    {
        Ok((decode(&text)?, Vec::new()))
    } else {
        crate::svg::import(&text)
    }
}
fn sibling(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(suffix);
    path.with_file_name(name)
}
#[cfg(windows)]
fn replace_file(source: &Path, destination: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    extern "system" {
        fn MoveFileExW(from: *const u16, to: *const u16, flags: u32) -> i32;
    }
    let a: Vec<_> = source.as_os_str().encode_wide().chain(Some(0)).collect();
    let b: Vec<_> = destination
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();
    if unsafe { MoveFileExW(a.as_ptr(), b.as_ptr(), 0x1 | 0x8) } == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}
#[cfg(not(windows))]
fn replace_file(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::rename(source, destination)
}
pub fn atomic_write(path: &Path, data: &[u8], backup: bool) -> Result<(), String> {
    if data.len() > LIMIT {
        return Err("Documento excede32MiB".into());
    }
    let temp = sibling(
        path,
        &format!(
            ".tmp-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ),
    );
    let operation = (|| -> std::io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        file.write_all(data)?;
        file.sync_all()?;
        drop(file);
        if backup && path.exists() {
            let old = fs::read(path)?;
            atomic_write(&sibling(path, ".bak"), &old, false).map_err(std::io::Error::other)?;
        }
        replace_file(&temp, path)
    })();
    if operation.is_err() {
        let _ = fs::remove_file(&temp);
    }
    operation.map_err(|e| e.to_string())
}
pub fn validated_encode(doc: &Document) -> Result<String, String> {
    let xml = encode(doc);
    decode(&xml)?;
    Ok(xml)
}
pub fn save(path: &Path, doc: &Document) -> Result<(), String> {
    let data = match path
        .extension()
        .and_then(|v| v.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("artdraw") => validated_encode(doc)?,
        Some("svg") if doc.pages.len() == 1 => crate::svg::export(doc),
        Some("svg") => return Err("SVG representa uma página. Salve o documento em .artdraw ou use Exportar página ativa como SVG.".into()),
        _ => return Err("Use a extensão .artdraw ou .svg".into()),
    };
    atomic_write(path, data.as_bytes(), true)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_roundtrip_all_geometry_and_state() {
        let mut d = Document::demo();
        d.layers[0].name = "A & B \"texto\"".into();
        d.objects[0].locked = true;
        d.objects[1].visible = false;
        d.objects[2].group = Some(99);
        d.objects[3].transform = Affine::rotate(0.1234567890123456);
        if let Geometry::Text { text, .. } = &mut d.objects[2].geometry {
            *text = "ação & <texto>\nlinha\t2".into();
        }
        let mut p = BezPath::new();
        p.move_to((0.1, 0.2));
        p.curve_to((1.2, 3.4), (5.6, 7.8), (9.1, 2.3));
        p.close_path();
        d.add_object(Object::new(
            "composto",
            Geometry::Compound(p),
            None,
            None,
            0.,
        ));
        assert_eq!(decode(&encode(&d)).unwrap(), d);
    }
    #[test]
    fn reject_future_version_and_duplicate_ids() {
        let d = Document::demo();
        assert!(
            decode(&encode(&d).replace("<artdraw version=\"3\"", "<artdraw version=\"99\""))
                .is_err()
        );
        let mut bad = d.clone();
        bad.objects[1].id = bad.objects[0].id;
        assert!(decode(&encode(&bad)).is_err());
    }
    #[test]
    fn reject_nonfinite_and_unknown_geometry() {
        let d = Document::demo();
        assert!(decode(&encode(&d).replace("width=\"960\"", "width=\"NaN\"")).is_err());
        assert!(decode(&encode(&d).replace("<ellipse ", "<future ")).is_err());
    }
    #[test]
    fn ids_survive_reorder_and_deletion() {
        let mut d = Document::demo();
        let id = d.objects[0].id;
        crate::editing::reorder(&mut d, &[0], true);
        assert_eq!(d.objects.last().unwrap().id, id);
        let removed = d.objects.pop().unwrap().id;
        let i = d.add_object(Object::new(
            "novo",
            Geometry::Rectangle(Rect::ZERO),
            None,
            None,
            0.,
        ));
        assert_ne!(d.objects[i].id, removed);
        assert_eq!(decode(&encode(&d)).unwrap(), d);
    }
    #[test]
    fn atomic_save_keeps_previous_backup() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/test-work")
            .join(format!(
                "artdraw-save-{}-{}",
                std::process::id(),
                SERIAL.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(&dir).unwrap();
        let p = dir.join("teste.artdraw");
        let mut d = Document::demo();
        save(&p, &d).unwrap();
        let before = fs::read(&p).unwrap();
        d.objects[0].name = "segunda versão".into();
        save(&p, &d).unwrap();
        assert_eq!(fs::read(sibling(&p, ".bak")).unwrap(), before);
        assert_eq!(read(&p).unwrap().0, d);
        let safe = fs::read(&p).unwrap();
        d.objects[1].id = d.objects[0].id;
        assert!(save(&p, &d).is_err());
        assert_eq!(fs::read(&p).unwrap(), safe);
        for f in fs::read_dir(&dir).unwrap() {
            fs::remove_file(f.unwrap().path()).unwrap();
        }
        fs::remove_dir(dir).unwrap();
    }
}

#[cfg(test)]
mod page_tests {
    use super::*;
    #[test]
    fn multipage_roundtrip_keeps_all_pages_and_independent_layers() {
        let mut d = Document::demo();
        let id = d.add_page(true).unwrap();
        d.width = 321.;
        d.height = 123.;
        d.name = "Capa ação & detalhe".into();
        d.layers[0].name = "Outra camada".into();
        d.objects[0].transform = Affine::rotate(0.123);
        d.move_page(-1);
        assert_eq!(d.active_page, id);
        assert_eq!(decode(&encode(&d)).unwrap(), d);
    }
    #[test]
    fn migrate_schema_one_without_changing_geometry_or_ids() {
        let legacy = include_str!("../tests/fixtures/legacy-v1.artdraw");
        let d = decode(legacy).unwrap();
        assert_eq!(d.pages.len(), 1);
        assert_eq!(d.active_page, 1);
        assert_eq!(d.objects.len(), 1);
        assert_eq!(d.objects[0].id, 1);
        assert_eq!(d.objects[0].transform, Affine::translate((1., 0.)));
        assert_eq!(decode(&encode(&d)).unwrap(), d);
    }
    #[test]
    fn reject_broken_page_references_and_cross_page_ids() {
        let mut d = Document::demo();
        d.add_page(true).unwrap();
        let xml = encode(&d);
        assert!(decode(&xml.replace("active-page=\"2\"", "active-page=\"999\"")).is_err());
        assert!(decode(&xml.replace("next-page-id=\"3\"", "next-page-id=\"2\"")).is_err());
        d.pages[1].id = d.pages[0].id;
        assert!(decode(&encode(&d)).is_err());
        d.pages[1].id = 2;
        d.pages[1].objects[0].id = d.pages[0].objects[0].id;
        assert!(decode(&encode(&d)).is_err());
        d.pages.clear();
        assert!(decode(&encode(&d)).is_err());
    }
    #[test]
    fn svg_save_cannot_silently_drop_other_pages() {
        let mut d = Document::demo();
        d.add_page(true).unwrap();
        let destination = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/test-work/multipage-must-not-write.svg");
        assert!(save(&destination, &d).unwrap_err().contains("página"));
        assert!(!destination.exists());
    }
    #[test]
    fn native_v3_nested_groups_roundtrip_and_legacy_v2() {
        let mut d = Document::demo();
        crate::editing::group(&mut d, &[0, 1]);
        crate::editing::group(&mut d, &[0, 1, 2, 3]);
        d.add_page(true).unwrap();
        assert_eq!(decode(&validated_encode(&d).unwrap()).unwrap(), d);
        let old = decode(include_str!("../tests/fixtures/legacy-v2.artdraw")).unwrap();
        assert_eq!(old.pages.len(), 3);
        assert!(old.pages.iter().all(|p| p.group_parents.is_empty()));
        assert_eq!(decode(&encode(&old)).unwrap(), old);
        let child = *d.group_parents.keys().next().unwrap();
        let parent = d.group_parents[&child];
        d.group_parents.insert(parent, child);
        assert!(validated_encode(&d).is_err());
    }
}
