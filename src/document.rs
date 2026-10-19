use vello::kurbo::{Affine, BezPath, Ellipse, PathEl, Point, Rect, Shape as KurboShape, Vec2};

pub type Color = [u8; 4];
pub const BLUE: Color = [58, 130, 246, 255];
pub const INK: Color = [30, 35, 44, 255];
pub const FONT: &[u8] = epaint_default_fonts::UBUNTU_LIGHT;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub p: Point,
    pub incoming: Point,
    pub outgoing: Point,
}
impl Node {
    pub fn corner(p: Point) -> Self {
        Self {
            p,
            incoming: p,
            outgoing: p,
        }
    }
    pub fn translate(&mut self, delta: Vec2) {
        self.p += delta;
        self.incoming += delta;
        self.outgoing += delta;
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Geometry {
    Rectangle(Rect),
    Compound(BezPath),
    Ellipse(Rect),
    Path {
        nodes: Vec<Node>,
        closed: bool,
    },
    Text {
        position: Point,
        text: String,
        size: f64,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub id: u64,
    pub name: String,
    pub geometry: Geometry,
    pub transform: Affine,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f64,
    pub evenodd: bool,
    pub layer: u64,
    pub group: Option<u64>,
    pub visible: bool,
    pub locked: bool,
}
impl Object {
    pub fn new(
        name: &str,
        geometry: Geometry,
        fill: Option<Color>,
        stroke: Option<Color>,
        width: f64,
    ) -> Self {
        Self {
            id: 0,
            name: name.into(),
            geometry,
            transform: Affine::IDENTITY,
            fill,
            stroke,
            stroke_width: width,
            evenodd: false,
            layer: 1,
            group: None,
            visible: true,
            locked: false,
        }
    }
    pub fn local_path(&self) -> BezPath {
        match &self.geometry {
            Geometry::Rectangle(r) => r.to_path(0.05),
            Geometry::Compound(path) => path.clone(),
            Geometry::Ellipse(r) => Ellipse::from_rect(*r).to_path(0.05),
            Geometry::Text {
                position,
                text,
                size,
            } => text_path(*position, text, *size),
            Geometry::Path { nodes, closed } => {
                let mut path = BezPath::new();
                if let Some(first) = nodes.first() {
                    path.move_to(first.p);
                    for pair in nodes.windows(2) {
                        path.curve_to(pair[0].outgoing, pair[1].incoming, pair[1].p);
                    }
                    if *closed && nodes.len() > 1 {
                        let last = nodes.last().unwrap();
                        path.curve_to(last.outgoing, first.incoming, first.p);
                        path.close_path();
                    }
                }
                path
            }
        }
    }
    pub fn path(&self) -> BezPath {
        self.transform * self.local_path()
    }
    pub fn bounds(&self) -> Rect {
        self.path().bounding_box()
    }
    pub fn translate(&mut self, d: Vec2) {
        self.transform = Affine::translate(d) * self.transform;
    }
    pub fn resize_from(&mut self, original: &Object, from: Rect, to: Rect) {
        let sx = to.width().max(0.1) / from.width().max(0.1);
        let sy = to.height().max(0.1) / from.height().max(0.1);
        self.transform = Affine::translate((to.x0, to.y0))
            * Affine::scale_non_uniform(sx, sy)
            * Affine::translate((-from.x0, -from.y0))
            * original.transform;
    }
    pub fn hit(&self, p: Point, tolerance: f64) -> bool {
        let path = self.path();
        if !path
            .bounding_box()
            .inflate(tolerance, tolerance)
            .contains(p)
        {
            return false;
        }
        let winding = path.winding(p);
        if self.fill.is_some()
            && (if self.evenodd {
                winding.abs() % 2 == 1
            } else {
                winding != 0
            })
        {
            return true;
        }
        let mut start = Point::ZERO;
        let mut prev = Point::ZERO;
        let mut hit = false;
        let width_scale = self.transform.determinant().abs().sqrt();
        let limit = tolerance + self.stroke_width * width_scale * 0.5;
        vello::kurbo::flatten(path.iter(), 0.35, |el| match el {
            PathEl::MoveTo(q) => {
                start = q;
                prev = q;
            }
            PathEl::LineTo(q) => {
                hit |= segment_distance(p, prev, q) <= limit;
                prev = q;
            }
            PathEl::ClosePath => {
                hit |= segment_distance(p, prev, start) <= limit;
                prev = start;
            }
            _ => {}
        });
        hit && (self.stroke.is_some() || matches!(self.geometry, Geometry::Path { .. }))
    }
    pub fn convert_to_path(&mut self) {
        let parts = objects_from_path(
            &self.local_path(),
            self.fill,
            self.stroke,
            self.stroke_width,
        );
        if parts.len() == 1 {
            self.geometry = parts.into_iter().next().unwrap().geometry;
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Layer {
    pub id: u64,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
}
impl Layer {
    pub fn base() -> Self {
        Self {
            id: 1,
            name: "Camada 1".into(),
            visible: true,
            locked: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    pub id: u64,
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub objects: Vec<Object>,
    pub layers: Vec<Layer>,
    pub active_layer: u64,
    /// Only child-to-parent edges; roots are implicit and IDs are local to a page.
    pub group_parents: std::collections::BTreeMap<u64, u64>,
}
impl Default for Page {
    fn default() -> Self {
        Self {
            id: 1,
            name: "Página 1".into(),
            width: 794.,
            height: 1123.,
            objects: Vec::new(),
            layers: vec![Layer::base()],
            active_layer: 1,
            group_parents: Default::default(),
        }
    }
}
impl Page {
    pub fn group_chain(&self, group: u64) -> Vec<u64> {
        let mut chain = vec![group];
        while let Some(&parent) = self.group_parents.get(chain.last().unwrap()) {
            if chain.len() >= 64 || chain.contains(&parent) {
                break;
            }
            chain.push(parent);
        }
        chain
    }
    pub fn root_group(&self, index: usize) -> Option<u64> {
        self.objects
            .get(index)?
            .group
            .map(|g| *self.group_chain(g).last().unwrap())
    }
    pub fn prune_groups(&mut self) {
        let used: std::collections::BTreeSet<_> = self
            .objects
            .iter()
            .filter_map(|o| o.group)
            .flat_map(|g| self.group_chain(g))
            .collect();
        self.group_parents.retain(|g, _| used.contains(g));
    }
    pub fn validate_groups(&self) -> Result<(), String> {
        if self.group_parents.len() > 50000 {
            return Err("Limite de grupos excedido".into());
        }
        // Validate every edge, including unreachable cycles, before traversing object chains.
        for (&child, &parent) in &self.group_parents {
            if child == 0 || parent == 0 {
                return Err("ID de grupo inválido".into());
            }
            let mut seen = std::collections::BTreeSet::new();
            let mut current = child;
            loop {
                if !seen.insert(current) {
                    return Err("Ciclo na hierarquia de grupos".into());
                }
                if seen.len() > 64 {
                    return Err("Hierarquia excede64 níveis".into());
                }
                match self.group_parents.get(&current) {
                    Some(&p) => current = p,
                    None => break,
                }
            }
        }
        let mut used = std::collections::BTreeSet::new();
        let mut layers = std::collections::BTreeMap::new();
        // Legacy schemas allowed flat groups across layers. Preserve their data on migration.
        let nested_roots: std::collections::BTreeSet<_> = self
            .group_parents
            .keys()
            .map(|&g| *self.group_chain(g).last().unwrap())
            .collect();
        for o in &self.objects {
            if let Some(g) = o.group {
                if g == 0 {
                    return Err("ID de grupo inválido".into());
                }
                let chain = self.group_chain(g);
                let root = *chain.last().unwrap();
                if nested_roots.contains(&root)
                    && layers
                        .insert(root, o.layer)
                        .is_some_and(|old| old != o.layer)
                {
                    return Err("Grupo contém objetos de camadas diferentes".into());
                }
                used.extend(chain);
            }
        }
        if self.group_parents.keys().any(|g| !used.contains(g)) {
            return Err("Aresta de grupo sem objetos descendentes".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub pages: Vec<Page>,
    pub active_page: u64,
    pub next_page_id: u64,
    pub next_object_id: u64,
}
impl Default for Document {
    fn default() -> Self {
        Self {
            pages: vec![Page::default()],
            active_page: 1,
            next_page_id: 2,
            next_object_id: 1,
        }
    }
}
// Existing editing/rendering commands operate on the active page only.
impl std::ops::Deref for Document {
    type Target = Page;
    fn deref(&self) -> &Page {
        self.pages
            .iter()
            .find(|p| p.id == self.active_page)
            .expect("active page invariant")
    }
}
impl std::ops::DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Page {
        self.pages
            .iter_mut()
            .find(|p| p.id == self.active_page)
            .expect("active page invariant")
    }
}
impl Document {
    pub fn with_size(width: f64, height: f64) -> Self {
        let mut d = Self::default();
        d.width = width;
        d.height = height;
        d
    }
    pub fn content_eq(&self, other: &Self) -> bool {
        self.pages == other.pages
            && self.next_page_id == other.next_page_id
            && self.next_object_id == other.next_object_id
    }
    pub fn total_objects(&self) -> usize {
        self.pages.iter().map(|p| p.objects.len()).sum()
    }
    pub fn activate_page(&mut self, id: u64) -> bool {
        if self.pages.iter().any(|p| p.id == id) {
            self.active_page = id;
            true
        } else {
            false
        }
    }
    pub fn add_page(&mut self, duplicate: bool) -> Result<u64, String> {
        if self.pages.len() >= 1000 {
            return Err("Limite de1000 páginas atingido".into());
        }
        let index = self
            .pages
            .iter()
            .position(|p| p.id == self.active_page)
            .unwrap();
        let mut page = if duplicate {
            self.pages[index].clone()
        } else {
            Page {
                width: self.width,
                height: self.height,
                ..Page::default()
            }
        };
        let id = self.next_page_id;
        self.next_page_id = id.checked_add(1).ok_or("IDs de páginas esgotados")?;
        page.id = id;
        page.name = if duplicate {
            format!(
                "{} — cópia",
                page.name.chars().take(248).collect::<String>()
            )
        } else {
            format!("Página {id}")
        };
        for object in &mut page.objects {
            object.id = self.allocate_object_id();
        }
        self.pages.insert(index + 1, page);
        self.active_page = id;
        Ok(id)
    }
    pub fn delete_page(&mut self, id: u64) -> Result<(), String> {
        if self.pages.len() == 1 {
            return Err("O documento precisa manter uma página".into());
        }
        let index = self
            .pages
            .iter()
            .position(|p| p.id == id)
            .ok_or("Página ausente")?;
        self.pages.remove(index);
        if self.active_page == id {
            self.active_page = self.pages[index.min(self.pages.len() - 1)].id;
        }
        Ok(())
    }
    pub fn move_page(&mut self, delta: isize) -> bool {
        let index = self
            .pages
            .iter()
            .position(|p| p.id == self.active_page)
            .unwrap();
        let target = index as isize + delta;
        if target < 0 || target >= self.pages.len() as isize {
            return false;
        }
        self.pages.swap(index, target as usize);
        true
    }
    pub fn hit(&self, p: Point, tolerance: f64) -> Option<usize> {
        self.paint_order()
            .into_iter()
            .rev()
            .find(|&i| self.editable(i) && self.objects[i].hit(p, tolerance))
    }
    pub fn visible(&self, index: usize) -> bool {
        self.objects
            .get(index)
            .is_some_and(|o| o.visible && self.layers.iter().any(|l| l.id == o.layer && l.visible))
    }
    pub fn editable(&self, index: usize) -> bool {
        self.visible(index)
            && self.objects.get(index).is_some_and(|o| {
                !o.locked && self.layers.iter().any(|l| l.id == o.layer && !l.locked)
            })
    }
    pub fn paint_order(&self) -> Vec<usize> {
        self.layers
            .iter()
            .flat_map(|l| {
                self.objects
                    .iter()
                    .enumerate()
                    .filter(move |(_, o)| o.layer == l.id)
                    .map(|(i, _)| i)
            })
            .collect()
    }
    pub fn add_object(&mut self, mut object: Object) -> usize {
        object.layer = self.active_layer;
        object.id = self.allocate_object_id();
        let index = self.objects.len();
        self.objects.push(object);
        index
    }
    fn allocate_object_id(&mut self) -> u64 {
        let mut id = self.next_object_id.max(1);
        while self
            .pages
            .iter()
            .flat_map(|p| &p.objects)
            .any(|o| o.id == id)
        {
            id = id.checked_add(1).unwrap_or(1);
        }
        self.next_object_id = id.checked_add(1).unwrap_or(1);
        id
    }
    pub fn ensure_object_ids(&mut self) {
        let mut seen = std::collections::BTreeSet::new();
        for p in 0..self.pages.len() {
            for i in 0..self.pages[p].objects.len() {
                let old = self.pages[p].objects[i].id;
                if old == 0 || !seen.insert(old) {
                    let id = self.allocate_object_id();
                    self.pages[p].objects[i].id = id;
                    seen.insert(id);
                }
            }
        }
        if let Some(max) = self
            .pages
            .iter()
            .flat_map(|p| &p.objects)
            .map(|o| o.id)
            .max()
        {
            self.next_object_id = self.next_object_id.max(max.checked_add(1).unwrap_or(1));
        }
    }
    pub fn next_group(&self) -> u64 {
        let used: std::collections::BTreeSet<_> = self
            .objects
            .iter()
            .filter_map(|o| o.group)
            .chain(self.group_parents.keys().copied())
            .chain(self.group_parents.values().copied())
            .collect();
        let mut id = used
            .last()
            .copied()
            .unwrap_or(0)
            .checked_add(1)
            .unwrap_or(1);
        while used.contains(&id) {
            id = id.checked_add(1).unwrap_or(1);
        }
        id
    }
    pub fn demo() -> Self {
        let mut doc = Self::with_size(960., 680.);
        doc.objects.push(Object::new(
            "Cartão",
            Geometry::Rectangle(Rect::new(90., 95., 870., 585.)),
            Some([235, 241, 250, 255]),
            None,
            0.,
        ));
        doc.objects.push(Object::new(
            "Círculo",
            Geometry::Ellipse(Rect::new(610., 150., 800., 340.)),
            Some(BLUE),
            None,
            0.,
        ));
        doc.objects.push(Object::new(
            "Título",
            Geometry::Text {
                position: Point::new(145., 180.),
                text: "ArtDraw".into(),
                size: 78.,
            },
            Some(INK),
            None,
            0.,
        ));
        doc.objects.push(Object::new(
            "Legenda",
            Geometry::Text {
                position: Point::new(150., 290.),
                text: "Ideias em curvas.\nVetores sem limites.".into(),
                size: 29.,
            },
            Some([85, 101, 126, 255]),
            None,
            0.,
        ));
        let mut a = Node::corner(Point::new(150., 475.));
        a.outgoing = Point::new(350., 280.);
        let mut b = Node::corner(Point::new(760., 440.));
        b.incoming = Point::new(480., 650.);
        doc.objects.push(Object::new(
            "Bézier",
            Geometry::Path {
                nodes: vec![a, b],
                closed: false,
            },
            None,
            Some(BLUE),
            9.,
        ));
        doc.ensure_object_ids();
        doc
    }
}
pub fn segment_distance(p: Point, a: Point, b: Point) -> f64 {
    let d = b - a;
    let t = if d.hypot2() < 1e-12 {
        0.
    } else {
        ((p - a).dot(d) / d.hypot2()).clamp(0., 1.)
    };
    p.distance(a + d * t)
}
pub fn objects_from_path(
    path: &BezPath,
    fill: Option<Color>,
    stroke: Option<Color>,
    width: f64,
) -> Vec<Object> {
    let mut result = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();
    let flush = |nodes: &mut Vec<Node>, closed, result: &mut Vec<Object>| {
        if !nodes.is_empty() {
            result.push(Object::new(
                "Caminho",
                Geometry::Path {
                    nodes: std::mem::take(nodes),
                    closed,
                },
                fill,
                stroke,
                width,
            ));
        }
    };
    for el in path.elements() {
        match *el {
            PathEl::MoveTo(p) => {
                flush(&mut nodes, false, &mut result);
                nodes.push(Node::corner(p));
            }
            PathEl::LineTo(p) => nodes.push(Node::corner(p)),
            PathEl::CurveTo(c1, c2, p) => {
                if let Some(last) = nodes.last_mut() {
                    last.outgoing = c1;
                }
                let mut node = Node::corner(p);
                node.incoming = c2;
                nodes.push(node);
            }
            PathEl::QuadTo(c, p) => {
                if let Some(last) = nodes.last_mut() {
                    last.outgoing = last.p + (c - last.p) * (2. / 3.);
                    let mut node = Node::corner(p);
                    node.incoming = p + (c - p) * (2. / 3.);
                    nodes.push(node);
                }
            }
            PathEl::ClosePath => {
                if nodes.len() > 1 && nodes[0].p.distance(nodes.last().unwrap().p) < 1e-9 {
                    let last = nodes.pop().unwrap();
                    nodes[0].incoming = last.incoming;
                }
                flush(&mut nodes, true, &mut result);
            }
        }
    }
    flush(&mut nodes, false, &mut result);
    result
}
struct Outline {
    path: BezPath,
    scale: f64,
    x: f64,
    baseline: f64,
}
impl ttf_parser::OutlineBuilder for Outline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.move_to((
            self.x + x as f64 * self.scale,
            self.baseline - y as f64 * self.scale,
        ));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to((
            self.x + x as f64 * self.scale,
            self.baseline - y as f64 * self.scale,
        ));
    }
    fn quad_to(&mut self, x: f32, y: f32, x1: f32, y1: f32) {
        self.path.quad_to(
            (
                self.x + x as f64 * self.scale,
                self.baseline - y as f64 * self.scale,
            ),
            (
                self.x + x1 as f64 * self.scale,
                self.baseline - y1 as f64 * self.scale,
            ),
        );
    }
    fn curve_to(&mut self, x: f32, y: f32, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.path.curve_to(
            (
                self.x + x as f64 * self.scale,
                self.baseline - y as f64 * self.scale,
            ),
            (
                self.x + x1 as f64 * self.scale,
                self.baseline - y1 as f64 * self.scale,
            ),
            (
                self.x + x2 as f64 * self.scale,
                self.baseline - y2 as f64 * self.scale,
            ),
        );
    }
    fn close(&mut self) {
        self.path.close_path();
    }
}
pub fn text_path(position: Point, text: &str, size: f64) -> BezPath {
    let Ok(face) = ttf_parser::Face::parse(FONT, 0) else {
        return BezPath::new();
    };
    let mut out = Outline {
        path: BezPath::new(),
        scale: size / f64::from(face.units_per_em()),
        x: position.x,
        baseline: position.y + size,
    };
    for ch in text.chars() {
        if ch == '\n' {
            out.x = position.x;
            out.baseline += size * 1.3;
            continue;
        }
        if let Some(id) = face.glyph_index(ch) {
            face.outline_glyph(id, &mut out);
            out.x += f64::from(face.glyph_hor_advance(id).unwrap_or(500)) * out.scale;
        }
    }
    out.path
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cubic_bounds_include_extrema() {
        let p = BezPath::from_svg("M0 0 C0 100 100 100 100 0").unwrap();
        let objects = objects_from_path(&p, None, Some(INK), 1.);
        assert!((objects[0].bounds().y1 - 75.).abs() < 1e-6);
    }
    #[test]
    fn closed_cubic_preserves_last_handle() {
        let p = BezPath::from_svg("M0 0 L100 0 C100 50 0 50 0 0 Z").unwrap();
        let o = objects_from_path(&p, Some(BLUE), None, 0.);
        assert_eq!(o.len(), 1);
        assert!((o[0].bounds().y1 - 37.5).abs() < 1e-6);
    }
    #[test]
    fn hit_test_ellipse_rejects_bbox_corner() {
        let o = Object::new(
            "",
            Geometry::Ellipse(Rect::new(0., 0., 100., 100.)),
            Some(BLUE),
            None,
            0.,
        );
        assert!(o.hit(Point::new(50., 50.), 1.));
        assert!(!o.hit(Point::new(2., 2.), 1.));
    }
    #[test]
    fn resize_maps_bounds() {
        let original = Object::new(
            "",
            Geometry::Rectangle(Rect::new(10., 20., 110., 70.)),
            Some(BLUE),
            None,
            0.,
        );
        let mut resized = original.clone();
        let target = Rect::new(30., 40., 230., 140.);
        resized.resize_from(&original, original.bounds(), target);
        assert_eq!(resized.bounds(), target);
    }
    #[test]
    fn node_translation_preserves_tangents() {
        let mut n = Node {
            p: Point::new(1., 2.),
            incoming: Point::new(0., 1.),
            outgoing: Point::new(3., 5.),
        };
        let tangent = n.outgoing - n.p;
        n.translate(Vec2::new(20., -4.));
        assert_eq!(n.outgoing - n.p, tangent);
    }
    #[test]
    fn text_has_real_outlines_and_unicode() {
        let a = text_path(Point::ZERO, "Olá, ação!", 32.);
        assert!(a.elements().len() > 30);
        assert!(a.bounding_box().width() > 80.);
    }
    #[test]
    fn topmost_object_wins() {
        let mut d = Document::default();
        for _ in 0..2 {
            d.objects.push(Object::new(
                "",
                Geometry::Rectangle(Rect::new(0., 0., 100., 100.)),
                Some(BLUE),
                None,
                0.,
            ));
        }
        assert_eq!(d.hit(Point::new(20., 20.), 1.), Some(1));
    }
    #[test]
    fn conversion_preserves_transform_and_style() {
        let mut o = Object::new(
            "Nome",
            Geometry::Ellipse(Rect::new(10., 20., 110., 80.)),
            Some(BLUE),
            Some(INK),
            3.,
        );
        o.transform = Affine::translate((20., 30.)) * Affine::scale_non_uniform(2., 1.5);
        let bounds = o.bounds();
        let transform = o.transform;
        o.convert_to_path();
        assert_eq!(o.name, "Nome");
        assert_eq!(o.transform, transform);
        assert_eq!(o.stroke_width, 3.);
        assert!((o.bounds().width() - bounds.width()).abs() < 0.01);
    }
}

#[cfg(test)]
mod page_tests {
    use super::*;
    #[test]
    fn page_navigation_is_content_neutral_and_isolates_hit_testing() {
        let mut d = Document::demo();
        let first = d.active_page;
        let second = d.add_page(false).unwrap();
        let saved = d.clone();
        assert!(d.objects.is_empty());
        assert_eq!(d.hit(Point::new(700., 230.), 2.), None);
        assert!(d.activate_page(first));
        assert!(d.content_eq(&saved));
        assert_eq!(d.objects.len(), 5);
        assert_eq!(d.hit(Point::new(700., 230.), 2.), Some(1));
        assert!(d.activate_page(second));
        d.width = 123.;
        d.layers[0].name = "Outra".into();
        d.activate_page(first);
        assert_eq!(d.width, 960.);
        assert_eq!(d.layers[0].name, "Camada 1");
        assert!(!d.activate_page(999));
        assert_eq!(d.active_page, first);
    }
    #[test]
    fn duplicate_page_preserves_content_but_allocates_unique_ids() {
        let mut d = Document::demo();
        d.objects[0].group = Some(7);
        d.objects[1].group = Some(7);
        d.objects[0].locked = true;
        let original = d.pages[0].clone();
        let id = d.add_page(true).unwrap();
        assert_eq!(d.pages.len(), 2);
        assert_eq!(d.active_page, id);
        for (a, b) in original.objects.iter().zip(&d.objects) {
            let mut same = b.clone();
            same.id = a.id;
            assert_eq!(same, *a);
            assert_ne!(a.id, b.id);
        }
        let ids = d
            .pages
            .iter()
            .flat_map(|p| &p.objects)
            .map(|o| o.id)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(ids.len(), 10);
        assert_eq!(d.total_objects(), 10);
    }
    #[test]
    fn page_order_delete_and_counters_are_stable() {
        let mut d = Document::demo();
        let id = d.add_page(true).unwrap();
        let next = d.next_object_id;
        assert!(d.move_page(-1));
        assert_eq!(d.pages[0].id, id);
        assert!(!d.move_page(-1));
        d.delete_page(id).unwrap();
        assert_eq!(d.active_page, 1);
        assert!(d.delete_page(1).is_err());
        assert_eq!(d.next_object_id, next);
        let added = d.add_page(false).unwrap();
        assert!(added > id);
        let i = d.add_object(Object::new(
            "Novo",
            Geometry::Rectangle(Rect::ZERO),
            None,
            None,
            0.,
        ));
        assert_eq!(d.objects[i].id, next);
    }
}
