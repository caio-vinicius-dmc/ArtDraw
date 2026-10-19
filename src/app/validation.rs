use super::*;
#[derive(Clone)]
enum Action {
    Wait,
    DropDemo,
    DropStacking,
    DropInterleaved,
    FastRect,
    FastMember(f64, f64, f64, f64),
    PickLeaf,
    NodePointer(Option<bool>, f64, f64),
    InputText(&'static str),
    RecordNavigation,
    RightClick(f64, f64),
    FastKey(Key, bool),
    PressKey(Key, bool),
    Down(f64, f64),
    Move(f64, f64),
    Up(f64, f64),
    Palette(bool, bool),
    Text,
    Apply(bool),
    Check(u8),
    ShiftKey(Key),
    LayerKey(Key),
    ShiftDown(f64, f64),
    ShiftUp(f64, f64),
    Widget(&'static str, bool),
}
pub(super) struct Validation {
    actions: Vec<Action>,
    step: usize,
    pending: Option<u8>,
    pub(super) view: Option<(ERect, Pos2)>,
    pub(super) palette: Option<Pos2>,
    pub(super) apply: Option<Pos2>,
    results: Vec<String>,
    failures: usize,
    navigation: Option<(Document, usize)>,
    pub(super) widgets: std::collections::BTreeMap<&'static str, Pos2>,
    pub(super) order_enabled: [bool; 7],
}
impl Validation {
    pub(super) fn new() -> Self {
        use Action::*;
        let actions = vec![
            Wait,
            Wait,
            PressKey(Key::R, false),
            FastRect,
            Check(44),
            FastKey(Key::Z, false),
            Check(45),
            FastKey(Key::Z, true),
            Check(44),
            FastKey(Key::Z, false),
            Check(45),
            Wait,
            DropStacking,
            Wait,
            Wait,
            Check(73),
            PressKey(Key::V, false),
            Down(120., 120.),
            Up(120., 120.),
            PressKey(Key::PageUp, true),
            Check(74),
            PressKey(Key::PageUp, true),
            Check(75),
            PressKey(Key::PageUp, true),
            Check(76),
            PressKey(Key::End, true),
            Check(77),
            PressKey(Key::Home, true),
            Check(78),
            PressKey(Key::PageDown, true),
            Check(76),
            PressKey(Key::PageDown, true),
            Check(75),
            LayerKey(Key::PageDown),
            Check(77),
            LayerKey(Key::PageUp),
            Check(75),
            RecordNavigation,
            LayerKey(Key::PageUp),
            Check(81),
            Check(80),
            PressKey(Key::Z, true),
            PressKey(Key::Z, true),
            PressKey(Key::Z, true),
            PressKey(Key::Z, true),
            PressKey(Key::Z, true),
            PressKey(Key::Z, true),
            PressKey(Key::Z, true),
            PressKey(Key::Z, true),
            PressKey(Key::Z, true),
            Check(82),
            Down(120., 120.),
            Up(120., 120.),
            RecordNavigation,
            Widget("order-reverse", true),
            Widget("order-reverse", false),
            Check(81),
            ShiftDown(400., 330.),
            ShiftUp(400., 330.),
            RecordNavigation,
            Widget("order-reverse", true),
            Widget("order-reverse", false),
            Check(85),
            Check(86),
            PressKey(Key::Z, true),
            Check(82),
            PressKey(Key::Y, true),
            Check(88),
            PressKey(Key::Z, true),
            Check(82),
            DropInterleaved,
            Wait,
            Wait,
            Check(89),
            Down(120., 120.),
            Up(120., 120.),
            RecordNavigation,
            PressKey(Key::PageUp, true),
            Check(90),
            PressKey(Key::PageDown, true),
            Check(91),
            RecordNavigation,
            LayerKey(Key::PageUp),
            Check(92),
            Check(93),
            LayerKey(Key::PageDown),
            Check(91),
            Check(97),
            Down(400., 330.),
            Up(400., 330.),
            RecordNavigation,
            PressKey(Key::PageUp, true),
            Check(94),
            Check(95),
            PressKey(Key::Z, true),
            Check(96),
            PressKey(Key::Y, true),
            Check(94),
            Down(580., 120.),
            Up(580., 120.),
            Check(98),
            RecordNavigation,
            Widget("order-up", true),
            Widget("order-up", false),
            Widget("order-front", true),
            Widget("order-front", false),
            Check(81),
            PressKey(Key::Home, true),
            Check(81),
            PressKey(Key::Z, true),
            Check(82),
            Down(120., 120.),
            Up(120., 120.),
            RecordNavigation,
            Widget("relative-front", true),
            Widget("relative-front", false),
            Check(99),
            Down(700., 550.),
            Up(700., 550.),
            Check(99),
            Down(120., 120.),
            Up(120., 120.),
            Check(99),
            Down(550., 450.),
            Up(550., 450.),
            Check(99),
            PressKey(Key::Escape, false),
            Check(100),
            Widget("relative-behind", true),
            Widget("relative-behind", false),
            Check(99),
            RightClick(700., 550.),
            Check(100),
            Widget("relative-front", true),
            Widget("relative-front", false),
            Check(99),
            Down(400., 330.),
            Move(430., 360.),
            Up(430., 360.),
            Check(101),
            Check(104),
            Check(102),
            PressKey(Key::Z, true),
            Check(103),
            PressKey(Key::Y, true),
            Check(101),
            PressKey(Key::Z, true),
            Check(103),
            Down(120., 120.),
            Up(120., 120.),
            RecordNavigation,
            Widget("relative-behind", true),
            Widget("relative-behind", false),
            Check(99),
            Down(580., 120.),
            Up(580., 120.),
            Check(105),
            Check(102),
            PressKey(Key::Z, true),
            Check(103),
            Down(120., 120.),
            Up(120., 120.),
            RecordNavigation,
            Widget("relative-front", true),
            Widget("relative-front", false),
            PressKey(Key::R, false),
            Check(106),
            Widget("relative-front", true),
            Widget("relative-front", false),
            Check(99),
            PressKey(Key::Escape, false),
            Check(108),
            PressKey(Key::V, false),
            Widget("relative-front", true),
            Widget("relative-front", false),
            PressKey(Key::PageDown, false),
            Check(107),
            PressKey(Key::PageUp, false),
            Check(82),
            DropDemo,
            Wait,
            Check(42),
            PressKey(Key::N, true),
            Wait,
            Wait,
            Wait,
            Wait,
            PressKey(Key::R, false),
            Down(120., 150.),
            Move(320., 280.),
            Up(320., 280.),
            Check(0),
            PressKey(Key::V, false),
            Down(220., 210.),
            Move(270., 250.),
            Up(270., 250.),
            Check(1),
            Down(370., 320.),
            Move(470., 380.),
            Up(470., 380.),
            Check(2),
            PressKey(Key::Z, true),
            Check(3),
            PressKey(Key::Y, true),
            Check(2),
            PressKey(Key::E, false),
            Down(500., 180.),
            Move(670., 300.),
            Up(670., 300.),
            Check(4),
            Palette(false, true),
            Palette(false, false),
            Check(5),
            Palette(true, true),
            Palette(true, false),
            Check(6),
            PressKey(Key::F10, false),
            Check(7),
            Down(670., 240.),
            Move(690., 260.),
            Up(690., 260.),
            Check(8),
            PressKey(Key::P, false),
            Down(120., 450.),
            Move(170., 390.),
            Up(170., 390.),
            Down(450., 480.),
            Move(500., 520.),
            Up(500., 520.),
            PressKey(Key::Enter, false),
            Check(9),
            PressKey(Key::T, false),
            Down(280., 650.),
            Up(280., 650.),
            Wait,
            Text,
            Wait,
            Apply(true),
            Apply(false),
            Wait,
            Check(10),
            Check(11),
            PressKey(Key::V, false),
            Down(80., 80.),
            Up(80., 80.),
            Down(250., 250.),
            Up(250., 250.),
            ShiftDown(585., 240.),
            ShiftUp(585., 240.),
            Check(30),
            Down(60., 60.),
            Move(740., 740.),
            Up(740., 740.),
            Check(31),
            PressKey(Key::A, true),
            Check(12),
            PressKey(Key::G, true),
            Check(13),
            Down(250., 250.),
            Move(270., 270.),
            Up(270., 270.),
            Check(14),
            PressKey(Key::C, true),
            PressKey(Key::V, true),
            Check(15),
            ShiftKey(Key::G),
            Check(16),
            PressKey(Key::Z, true),
            Check(17),
            PressKey(Key::Y, true),
            Check(16),
            PressKey(Key::Y, false),
            Down(100., 800.),
            Move(260., 960.),
            Up(260., 960.),
            Check(18),
            PressKey(Key::S, false),
            Down(300., 800.),
            Move(440., 960.),
            Up(440., 960.),
            Check(19),
            PressKey(Key::L, false),
            Down(100., 1020.),
            Move(300., 1020.),
            Up(300., 1020.),
            Check(20),
            PressKey(Key::F5, false),
            Down(400., 800.),
            Move(410., 820.),
            Move(450., 840.),
            Move(470., 810.),
            Up(470., 810.),
            Check(21),
            PressKey(Key::I, false),
            Down(500., 800.),
            Move(650., 950.),
            Up(650., 950.),
            Check(22),
            Widget("layers-header", true),
            Widget("layers-header", false),
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            Widget("new-layer", true),
            Widget("new-layer", false),
            Wait,
            Check(23),
            PressKey(Key::R, false),
            Down(100., 150.),
            Move(200., 200.),
            Up(200., 200.),
            Check(24),
            Widget("layer-visible", true),
            Widget("layer-visible", false),
            Check(25),
            Widget("layer-visible", true),
            Widget("layer-visible", false),
            Widget("layer-locked", true),
            Widget("layer-locked", false),
            Check(26),
            Widget("layer-locked", true),
            Widget("layer-locked", false),
            Widget("layers-header", true),
            Widget("layers-header", false),
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            PressKey(Key::V, false),
            Down(150., 175.),
            Up(150., 175.),
            Widget("transforms-header", true),
            Widget("transforms-header", false),
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            Widget("rotate", true),
            Widget("rotate", false),
            Check(34),
            Widget("mirror-h", true),
            Widget("mirror-h", false),
            Check(35),
            Widget("transforms-header", true),
            Widget("transforms-header", false),
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            Widget("align-header", true),
            Widget("align-header", false),
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            Wait,
            Widget("align-page", true),
            Widget("align-page", false),
            Widget("align-center", true),
            Widget("align-center", false),
            Check(37),
            Widget("theme-menu", true),
            Widget("theme-menu", false),
            Wait,
            Widget("theme-light", true),
            Widget("theme-light", false),
            Wait,
            Check(27),
            Widget("theme-menu", true),
            Widget("theme-menu", false),
            Wait,
            Widget("theme-dark", true),
            Widget("theme-dark", false),
            Wait,
            Check(28),
            Check(29),
            Check(38),
            Check(39),
            Widget("new-page", true),
            Widget("new-page", false),
            Wait,
            Wait,
            Check(46),
            PressKey(Key::R, false),
            Down(100., 150.),
            Move(200., 200.),
            Up(200., 200.),
            Check(47),
            Widget("duplicate-page", true),
            Widget("duplicate-page", false),
            Wait,
            Wait,
            Check(48),
            Widget("page-manager", true),
            Widget("page-manager", false),
            Wait,
            Wait,
            Wait,
            Wait,
            Widget("page-name", true),
            Widget("page-name", false),
            PressKey(Key::A, true),
            InputText("Detalhe ação"),
            Widget("page-rename", true),
            Widget("page-rename", false),
            Check(49),
            Widget("page-landscape", true),
            Widget("page-landscape", false),
            Check(50),
            Widget("page-left", true),
            Widget("page-left", false),
            Check(51),
            Widget("page-delete", true),
            Widget("page-delete", false),
            Wait,
            Wait,
            Check(52),
            Widget("page-delete-confirm", true),
            Widget("page-delete-confirm", false),
            Wait,
            Wait,
            Check(53),
            Widget("page-close", true),
            Widget("page-close", false),
            Wait,
            Wait,
            PressKey(Key::Z, true),
            Check(54),
            PressKey(Key::Y, true),
            Check(53),
            PressKey(Key::Z, true),
            Check(54),
            RecordNavigation,
            PressKey(Key::PageUp, false),
            Wait,
            Wait,
            Check(55),
            Check(56),
            PressKey(Key::V, false),
            PressKey(Key::A, true),
            PressKey(Key::G, true),
            Check(57),
            RecordNavigation,
            PressKey(Key::G, true),
            Check(58),
            PressKey(Key::C, true),
            PressKey(Key::V, true),
            Check(59),
            Check(65),
            ShiftKey(Key::G),
            Check(60),
            PressKey(Key::Z, true),
            Check(59),
            Down(413., 191.),
            Up(413., 191.),
            Check(65),
            Widget("ungroup-all", true),
            Widget("ungroup-all", false),
            Check(61),
            PressKey(Key::Z, true),
            Check(59),
            Down(413., 191.),
            Up(413., 191.),
            Check(65),
            Check(62),
            Check(63),
            PressKey(Key::Delete, false),
            Check(64),
            PressKey(Key::Z, true),
            Check(59),
            RecordNavigation,
            FastMember(413., 191., 433., 211.),
            Check(66),
            Widget("clear-fill", true),
            Widget("clear-fill", false),
            Check(67),
            PressKey(Key::C, true),
            PressKey(Key::V, true),
            Check(68),
            PressKey(Key::Z, true),
            RecordNavigation,
            PickLeaf,
            PressKey(Key::Delete, false),
            Check(69),
            PressKey(Key::Z, true),
            Check(70),
            PickLeaf,
            RecordNavigation,
            PressKey(Key::F10, false),
            Check(71),
            RecordNavigation,
            NodePointer(Some(true), 0., 0.),
            NodePointer(None, 10., 10.),
            NodePointer(Some(false), 10., 10.),
            Check(72),
            PressKey(Key::Z, true),
            Check(70),
            Check(63),
            Check(83),
            Check(84),
        ];
        Self {
            actions,
            step: 0,
            pending: None,
            view: None,
            palette: None,
            apply: None,
            results: Vec::new(),
            failures: 0,
            navigation: None,
            widgets: std::collections::BTreeMap::new(),
            order_enabled: [false; 7],
        }
    }
    pub(super) fn recovery() -> Self {
        let mut v = Self::new();
        v.actions = vec![
            Action::Wait,
            Action::Wait,
            Action::Wait,
            Action::Wait,
            Action::Widget("recover", true),
            Action::Widget("recover", false),
            Action::Wait,
            Action::Check(40),
            Action::Check(38),
            Action::Check(39),
        ];
        v
    }
    pub(super) fn finished(&self) -> bool {
        self.step >= self.actions.len()
    }
    pub(super) fn passed(&self) -> bool {
        self.failures == 0 && self.finished()
    }
    pub(super) fn report(&self) -> String {
        format!(
            "{}\nInterface checks: {}\n{}\n",
            if self.passed() { "PASS" } else { "FAIL" },
            self.results.len(),
            self.results.join("\n")
        )
    }
    pub(super) fn inject(&mut self, app: &ArtDraw, raw: &mut egui::RawInput) {
        if self.finished() {
            return;
        }
        let Some((_, origin)) = self.view else {
            return;
        };
        let action = self.actions[self.step].clone();
        self.step += 1;
        raw.events.clear();
        raw.modifiers = egui::Modifiers::default();
        let point = |x, y| {
            egui::pos2(
                origin.x + (x * app.zoom) as f32,
                origin.y + (y * app.zoom) as f32,
            )
        };
        match action {
            Action::ShiftDown(x, y) => {
                raw.modifiers.shift = true;
                button(raw, point(x, y), PointerButton::Primary, true);
            }
            Action::ShiftUp(x, y) => {
                raw.modifiers.shift = true;
                button(raw, point(x, y), PointerButton::Primary, false);
            }
            Action::LayerKey(key) => {
                let modifiers = egui::Modifiers {
                    shift: true,
                    ..Default::default()
                };
                raw.modifiers = modifiers;
                for pressed in [true, false] {
                    raw.events.push(egui::Event::Key {
                        key,
                        physical_key: Some(key),
                        pressed,
                        repeat: false,
                        modifiers,
                    });
                }
            }
            Action::ShiftKey(key) => {
                let modifiers = egui::Modifiers {
                    ctrl: true,
                    command: true,
                    shift: true,
                    ..Default::default()
                };
                raw.modifiers = modifiers;
                for pressed in [true, false] {
                    raw.events.push(egui::Event::Key {
                        key,
                        physical_key: Some(key),
                        pressed,
                        repeat: false,
                        modifiers,
                    });
                }
            }
            Action::Widget(name, pressed) => {
                if let Some(&p) = self.widgets.get(name) {
                    button(raw, p, PointerButton::Primary, pressed);
                } else {
                    self.failures += 1;
                    self.results.push(format!("ERRO: controle ausente {name}"));
                }
            }
            Action::DropInterleaved => {
                let path = app
                    .smoke
                    .as_ref()
                    .unwrap()
                    .with_extension("interleaved-input.artdraw");
                if let Err(e) = std::fs::write(
                    &path,
                    include_str!("../../tests/fixtures/reverse-interleaved.artdraw"),
                ) {
                    self.failures += 1;
                    self.results.push(format!("ERRO fixture intercalada: {e}"));
                }
                raw.dropped_files.push(egui::DroppedFile {
                    path: Some(path),
                    ..Default::default()
                });
            }
            Action::DropStacking => {
                let mut d = Document::with_size(800., 600.);
                for (x, y, c) in [
                    (100., 100., [239, 68, 68, 255]),
                    (180., 150., [59, 130, 246, 255]),
                    (260., 200., [34, 197, 94, 255]),
                ] {
                    d.add_object(Object::new(
                        "Pilha",
                        Geometry::Rectangle(Rect::new(x, y, x + 180., y + 160.)),
                        Some(c),
                        None,
                        0.,
                    ));
                }
                d.layers.push(crate::document::Layer {
                    id: 2,
                    name: "Bloqueada".into(),
                    visible: true,
                    locked: true,
                });
                d.active_layer = 2;
                d.add_object(Object::new(
                    "Bloqueado",
                    Geometry::Rectangle(Rect::new(500., 400., 600., 500.)),
                    Some([100, 100, 100, 255]),
                    None,
                    0.,
                ));
                d.layers.push(crate::document::Layer {
                    id: 3,
                    name: "Superior".into(),
                    visible: true,
                    locked: false,
                });
                d.active_layer = 3;
                d.add_object(Object::new(
                    "Superior",
                    Geometry::Rectangle(Rect::new(550., 100., 650., 200.)),
                    Some([250, 204, 21, 255]),
                    None,
                    0.,
                ));
                d.active_layer = 1;
                d.add_page(false).unwrap();
                d.activate_page(1);
                let path = app
                    .smoke
                    .as_ref()
                    .unwrap()
                    .with_extension("order-input.artdraw");
                if let Err(e) = crate::storage::save(&path, &d) {
                    self.failures += 1;
                    self.results.push(format!("ERRO fixture ordem: {e}"));
                }
                raw.dropped_files.push(egui::DroppedFile {
                    path: Some(path),
                    ..Default::default()
                });
            }
            Action::DropDemo => {
                if let Some(path) = app
                    .smoke
                    .as_ref()
                    .map(|p| p.with_extension("input.artdraw"))
                {
                    if let Err(e) = crate::storage::save(&path, &Document::demo()) {
                        self.failures += 1;
                        self.results.push(format!("ERRO: fixture nativa: {e}"));
                    }
                    raw.dropped_files.push(egui::DroppedFile {
                        path: Some(path),
                        ..Default::default()
                    });
                }
            }
            Action::PickLeaf => {
                let path = app.doc.objects[27].path();
                let mut vertices = path.elements().iter().filter_map(|e| match e {
                    vello::kurbo::PathEl::MoveTo(p) | vello::kurbo::PathEl::LineTo(p) => Some(*p),
                    _ => None,
                });
                let a = vertices.next().unwrap();
                let b = vertices.next().unwrap();
                let p = point((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
                raw.modifiers = egui::Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                };
                button(raw, p, PointerButton::Primary, true);
                raw.modifiers = Default::default();
                button(raw, p, PointerButton::Primary, false);
            }
            Action::NodePointer(pressed, dx, dy) => {
                let object = &self.navigation.as_ref().unwrap().0.objects[27];
                let Geometry::Path { nodes, .. } = &object.geometry else {
                    panic!("Fixture de nós ausente")
                };
                let p = object.transform * nodes[0].p;
                let p = point(p.x + dx, p.y + dy);
                if let Some(pressed) = pressed {
                    button(raw, p, PointerButton::Primary, pressed);
                } else {
                    raw.events.push(egui::Event::PointerMoved(p));
                }
            }
            Action::FastMember(x, y, tx, ty) => {
                raw.modifiers = egui::Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                };
                button(raw, point(x, y), PointerButton::Primary, true);
                raw.modifiers = egui::Modifiers::default();
                raw.events.push(egui::Event::PointerMoved(point(tx, ty)));
                button(raw, point(tx, ty), PointerButton::Primary, false);
            }
            Action::FastRect => {
                button(raw, point(100., 150.), PointerButton::Primary, true);
                raw.events
                    .push(egui::Event::PointerMoved(point(200., 200.)));
                button(raw, point(200., 200.), PointerButton::Primary, false);
            }
            Action::FastKey(key, shift) => {
                let modifiers = egui::Modifiers {
                    ctrl: true,
                    command: true,
                    shift,
                    ..Default::default()
                };
                // Final aggregate modifiers are empty, as with a complete native chord in one frame.
                for pressed in [true, false] {
                    raw.events.push(egui::Event::Key {
                        key,
                        physical_key: Some(key),
                        pressed,
                        repeat: false,
                        modifiers,
                    });
                }
            }
            Action::InputText(text) => raw.events.push(egui::Event::Text(text.into())),
            Action::RightClick(x, y) => {
                button(raw, point(x, y), PointerButton::Secondary, true);
                button(raw, point(x, y), PointerButton::Secondary, false);
            }
            Action::RecordNavigation => self.navigation = Some((app.doc.clone(), app.undo.len())),
            Action::Wait => {}
            Action::Check(n) => self.pending = Some(n),
            Action::PressKey(key, command) => {
                let modifiers = egui::Modifiers {
                    ctrl: command,
                    command,
                    ..Default::default()
                };
                raw.modifiers = modifiers;
                for pressed in [true, false] {
                    raw.events.push(egui::Event::Key {
                        key,
                        physical_key: Some(key),
                        pressed,
                        repeat: false,
                        modifiers,
                    });
                }
            }
            Action::Down(x, y) => button(raw, point(x, y), PointerButton::Primary, true),
            Action::Up(x, y) => button(raw, point(x, y), PointerButton::Primary, false),
            Action::Move(x, y) => raw.events.push(egui::Event::PointerMoved(point(x, y))),
            Action::Palette(stroke, pressed) => {
                if let Some(p) = self.palette {
                    button(
                        raw,
                        p,
                        if stroke {
                            PointerButton::Secondary
                        } else {
                            PointerButton::Primary
                        },
                        pressed,
                    );
                }
            }
            Action::Text => raw.events.push(egui::Event::Text("Teste ação".into())),
            Action::Apply(pressed) => {
                if let Some(p) = self.apply {
                    button(raw, p, PointerButton::Primary, pressed);
                }
            }
        }
    }
    pub(super) fn check(&mut self, app: &ArtDraw) {
        let Some(n) = self.pending.take() else {
            return;
        };
        let doc = &app.doc;
        let paint_ids: Vec<_> = doc
            .paint_order()
            .iter()
            .map(|&i| doc.objects[i].id)
            .collect();
        let near = |a: f64, b: f64| (a - b).abs() < 1.;
        let (label,ok)=match n {
            0=>("mouse: desenhar retângulo",doc.objects.len()==1&&near(doc.objects[0].bounds().width(),200.)&&near(doc.objects[0].bounds().height(),130.)),
            1=>("mouse: mover seleção",doc.objects.first().is_some_and(|o|near(o.bounds().x0,170.)&&near(o.bounds().y0,190.))),
            2=>("mouse: redimensionar seleção",doc.objects.first().is_some_and(|o|near(o.bounds().width(),300.)&&near(o.bounds().height(),190.))),
            3=>("atalho: desfazer",doc.objects.first().is_some_and(|o|near(o.bounds().width(),200.))),
            4=>("mouse: desenhar elipse",doc.objects.len()==2&&matches!(doc.objects[1].geometry,Geometry::Ellipse(_))),
            5=>("paleta: esquerdo aplica fill",doc.objects.get(1).is_some_and(|o|o.fill==Some([239,68,68,255]))),
            6=>("paleta: direito aplica stroke",doc.objects.get(1).is_some_and(|o|o.stroke==Some([239,68,68,255]))),
            7=>("F10: converter elipse em nós",doc.objects.get(1).is_some_and(|o|matches!(&o.geometry,Geometry::Path{nodes,..}if nodes.len()>=4))),
            8=>("F10: arrastar nó",doc.objects.get(1).is_some_and(|o|matches!(&o.geometry,Geometry::Path{nodes,..}if nodes.iter().any(|n|near((o.transform*n.p).x,690.)&&near((o.transform*n.p).y,260.))))),
            9=>("caneta: criar cúbica por arrasto e Enter",doc.objects.len()==3&&matches!(&doc.objects[2].geometry,Geometry::Path{nodes,closed:false}if nodes.len()==2&&nodes[0].p.distance(nodes[0].outgoing)>30.)),
            10=>("texto: inserir Unicode pelo editor",doc.objects.len()==4&&matches!(&doc.objects[3].geometry,Geometry::Text{text,..}if text=="Teste ação")),
12=>("Ctrl+A: seleção coletiva",app.selected_indices().len()==4),
13=>("Ctrl+G: grupo simples",doc.objects.len()==4&&doc.objects.iter().all(|o|o.group.is_some()&&o.group==doc.objects[0].group)),
14=>("arrasto: mover grupo",doc.objects.first().is_some_and(|o|near(o.bounds().x0,190.)&&near(o.bounds().y0,210.))),
15=>("Ctrl+C/V: copiar grupo sem colisão",doc.objects.len()==8&&app.selected_indices().len()==4&&doc.objects[0].group!=doc.objects[4].group),
16=>("Ctrl+Shift+G/redo: desagrupar",doc.objects.len()==8&&doc.objects[4..].iter().all(|o|o.group.is_none())),
17=>("undo restaura grupo",doc.objects.len()==8&&doc.objects[4..].iter().all(|o|o.group.is_some())),
18=>("polígono por ponteiro",doc.objects.len()==9&&matches!(&doc.objects[8].geometry,Geometry::Path{nodes,closed:true}if nodes.len()==5)),
19=>("estrela por ponteiro",doc.objects.len()==10&&matches!(&doc.objects[9].geometry,Geometry::Path{nodes,closed:true}if nodes.len()==10)),
20=>("linha horizontal por ponteiro",doc.objects.len()==11&&doc.objects[10].bounds().height()<1.),
21=>("desenho livre por ponteiro",doc.objects.len()==12&&matches!(&doc.objects[11].geometry,Geometry::Path{nodes,closed:false}if nodes.len()>=4)),
22=>("espiral por ponteiro",doc.objects.len()==13&&matches!(&doc.objects[12].geometry,Geometry::Path{nodes,closed:false}if nodes.len()>100)),
23=>("UI: criar camada",doc.layers.len()==2&&doc.active_layer==2),
24=>("desenho na camada ativa",doc.objects.len()==14&&doc.objects[13].layer==2),
25=>("UI: ocultar camada",doc.objects.len()==14&&!doc.visible(13)),
26=>("UI: bloquear camada",doc.objects.len()==14&&!doc.editable(13)&&doc.visible(13)),
27=>("UI: tema claro",app.theme==ThemeKind::Light),28=>("UI: tema escuro",app.theme==ThemeKind::Dark),
34=>("UI: giro numérico 15°",doc.objects.get(13).is_some_and(|o|near(o.bounds().width(),100.*15f64.to_radians().cos()+50.*15f64.to_radians().sin()))),
35=>("UI: espelho horizontal",doc.objects.get(13).is_some_and(|o|o.transform.determinant()<0.)),
37=>("UI: alinhar ao centro da página",doc.objects.get(13).is_some_and(|o|near(o.bounds().center().x,doc.width/2.))),
30=>("Shift+clique: seleção aditiva",app.selected_indices()==vec![0,1]),31=>("marquee: seleção por contenção",app.selected_indices().len()==4),
46=>("Páginas: adicionar vazia sem alterar a primeira",doc.pages.len()==2&&doc.active_page==2&&doc.objects.is_empty()&&doc.pages[0].objects.len()==14),
47=>("Páginas: desenho isolado na segunda",doc.active_page==2&&doc.objects.len()==1&&doc.pages[0].objects.len()==14),
48=>("Páginas: duplicar com IDs globais únicos",doc.pages.len()==3&&doc.active_page==3&&doc.objects.len()==1&&doc.pages.iter().flat_map(|p|&p.objects).map(|o|o.id).collect::<std::collections::BTreeSet<_>>().len()==16),
49=>("Páginas: renomear com Unicode",doc.name=="Detalhe ação"&&doc.pages[0].name=="Página 1"),
50=>("Páginas: orientação independente",doc.width>doc.height&&doc.pages[0].width<doc.pages[0].height),
51=>("Páginas: ordenar preservando identidades",doc.pages.iter().map(|p|p.id).collect::<Vec<_>>()==vec![1,3,2]),
52=>("Páginas: excluir requer confirmação",doc.pages.len()==3&&app.pending_page_delete==Some(3)),
53=>("Páginas: exclusão e redo preservam vizinha",doc.pages.len()==2&&doc.active_page==2&&doc.objects.len()==1&&doc.pages[0].objects.len()==14),
54=>("Páginas: undo restaura conteúdo, nome e posição",doc.pages.len()==3&&doc.active_page==3&&doc.name=="Detalhe ação"&&doc.objects.len()==1&&doc.pages[1].id==3),
55=>("Páginas: navegação não muda conteúdo/histórico",doc.active_page==1&&doc.objects.len()==14&&self.navigation.as_ref().is_some_and(|(d,h)|doc.content_eq(d)&&app.undo.len()==*h)),
57=>("Grupos: agrupar14 folhas preserva subgrupo",doc.objects.len()==14&&doc.group_parents.len()==1&&app.selected_indices().len()==14&&doc.objects.iter().enumerate().all(|(i,_)|doc.root_group(i)==doc.root_group(0))),
58=>("Grupos: agrupar unidade única não cria nível nem histórico",self.navigation.as_ref().is_some_and(|(d,h)|d==doc&&*h==app.undo.len())),
59=>("Grupos: copiar/restaurar árvore sem colisões",doc.objects.len()==28&&doc.group_parents.len()==2&&doc.root_group(0)!=doc.root_group(14)&&doc.validate_groups().is_ok()),
60=>("Grupos: separar externo mantém subgrupo de4 folhas",doc.group_parents.len()==1&&app.selected_indices().iter().filter(|&&i|doc.objects[i].group.is_some()).count()==4),
61=>("Grupos: botão separar tudo remove árvore da cópia",doc.group_parents.len()==1&&app.selected_indices().iter().all(|&i|doc.objects[i].group.is_none())),
62=>{let mut active=doc.clone();active.pages.retain(|p|p.id==active.active_page);("Grupos: SVG ativo conserva árvore, estilos e identidades",svg::import(&svg::export(&active)).is_ok_and(|r|r.0==active))},
63=>{let path=app.smoke.as_ref().unwrap().with_extension("nested.artdraw");("Grupos: salvar/reabrir nativo com múltiplos níveis",crate::storage::save(&path,doc).and_then(|_|crate::storage::read(&path)).is_ok_and(|r|r.0==*doc))},
85=>("Inversão: botão troca apenas duas unidades e mantém seleção/atributos",paint_ids==[3,2,1,4,5]&&app.selected_indices().iter().map(|&i|doc.objects[i].id).collect::<std::collections::BTreeSet<_>>()==[1,3].into_iter().collect()&&self.navigation.as_ref().is_some_and(|(d,h)|app.undo.len()==h+1&&doc.layers==d.layers&&doc.group_parents==d.group_parents&&doc.objects.iter().all(|o|d.objects.iter().any(|v|v==o)))),
86=>{let file=app.smoke.as_ref().unwrap().with_extension("reverse.artdraw");let native=crate::storage::save(&file,doc).and_then(|_|crate::storage::read(&file)).is_ok_and(|r|r.0==*doc);let mut active=doc.clone();active.pages.retain(|p|p.id==active.active_page);("Inversão: persistir ordem no nativo e SVG",native&&svg::import(&svg::export(&active)).is_ok_and(|r|r.0==active))},
88=>("Inversão: redo restaura pilha e atributos",paint_ids==[3,2,1,4,5]&&self.navigation.as_ref().is_some_and(|(d,h)|app.undo.len()==h+1&&doc.layers==d.layers&&doc.objects.iter().all(|o|d.objects.iter().any(|v|v==o)))),
83=>{let file=app.smoke.as_ref().unwrap().with_extension("pdf");let before=doc.clone();let result=crate::pdf::save(&file,doc,true).and_then(|_|std::fs::read(&file).map_err(|e|e.to_string()));("PDF: documento multipágina salvo sem alterar nativo",result.is_ok_and(|b|b.starts_with(b"%PDF-1.4")&&String::from_utf8_lossy(&b).contains("/Count 3"))&&*doc==before)},
84=>{let file=app.smoke.as_ref().unwrap().with_extension("active.pdf");let result=crate::pdf::save(&file,doc,false).and_then(|_|std::fs::read(&file).map_err(|e|e.to_string()));("PDF: exportar só página ativa",result.is_ok_and(|b|b.starts_with(b"%PDF-1.4")&&String::from_utf8_lossy(&b).contains("/Count 1")))},
99=>("Alvo: escolha e clique inválido preservam documento/seleção/histórico",app.pending_relative.is_some()&&app.selected_indices().iter().map(|&i|doc.objects[i].id).collect::<Vec<_>>()==[1]&&self.navigation.as_ref().is_some_and(|(d,h)|d==doc&&app.undo.len()==*h)),
100=>("Alvo: Esc/direito cancela sem perder seleção ou criar dirty",app.pending_relative.is_none()&&app.tool==Tool::Select&&app.selected_indices().iter().map(|&i|doc.objects[i].id).collect::<Vec<_>>()==[1]&&self.navigation.as_ref().is_some_and(|(d,h)|d==doc&&app.undo.len()==*h)),
101=>("Alvo: frente imediata e redo conservam atributos/pilha",paint_ids==[2,3,1,4,5]&&app.pending_relative.is_none()&&self.navigation.as_ref().is_some_and(|(d,h)|app.undo.len()==h+1&&doc.layers==d.layers&&doc.pages[1]==d.pages[1]&&doc.objects.iter().all(|o|d.objects.iter().any(|v|v==o)))),
102=>{let file=app.smoke.as_ref().unwrap().with_extension("relative.artdraw");let native=crate::storage::save(&file,doc).and_then(|_|crate::storage::read(&file)).is_ok_and(|r|r.0==*doc);let mut active=doc.clone();active.pages.retain(|p|p.id==active.active_page);("Alvo: salvar/reabrir nativo e SVG exatos",native&&svg::import(&svg::export(&active)).is_ok_and(|r|r.0==active))},
103=>("Alvo: undo restaura documento salvo exato",doc==&app.saved&&app.pending_relative.is_none()&&self.navigation.as_ref().is_some_and(|(d,h)|doc==d&&app.undo.len()==*h)),
104=>("Alvo: clique/arrasto mantém fonte selecionada sem mover geometria",app.pending_relative.is_none()&&!app.relative_click_active&&app.drag.is_none()&&app.selected_indices().iter().map(|&i|doc.objects[i].id).collect::<Vec<_>>()==[1]),
105=>("Alvo: atrás de outra camada transfere só fonte e mantém atributos",paint_ids==[2,3,4,1,5]&&doc.active_layer==3&&app.pending_relative.is_none()&&self.navigation.as_ref().is_some_and(|(d,h)|app.undo.len()==h+1&&doc.layers==d.layers&&doc.objects.iter().all(|o|d.objects.iter().find(|v|v.id==o.id).is_some_and(|v|{let mut expected=v.clone();if o.id==1{expected.layer=3;}*o==expected})))),
106=>("Alvo: trocar ferramenta cancela sem alteração",app.pending_relative.is_none()&&app.tool==Tool::Rectangle&&self.navigation.as_ref().is_some_and(|(d,h)|d==doc&&app.undo.len()==*h)),
108=>("Alvo: cancelar restaura ferramenta anterior",app.pending_relative.is_none()&&app.tool==Tool::Rectangle&&self.navigation.as_ref().is_some_and(|(d,h)|d==doc&&app.undo.len()==*h)),
107=>("Alvo: trocar página cancela sem alterar conteúdo/histórico",app.pending_relative.is_none()&&doc.active_page==2&&self.navigation.as_ref().is_some_and(|(d,h)|doc.content_eq(d)&&app.undo.len()==*h)),
94=>("Transferência: slots alheios, atributos e histórico preservados",doc.objects.iter().map(|o|o.id).collect::<Vec<_>>()==[1,4,2,3,5]&&doc.active_layer==3&&self.navigation.as_ref().is_some_and(|(d,h)|app.undo.len()==h+1&&doc.layers==d.layers&&doc.pages[1]==d.pages[1]&&doc.objects.iter().all(|o|d.objects.iter().find(|v|v.id==o.id).is_some_and(|v|{let mut expected=v.clone();if o.id==3{expected.layer=3;}*o==expected})))),
95=>{let file=app.smoke.as_ref().unwrap().with_extension("transfer.artdraw");("Transferência: salvar/reabrir representação e camadas exatas",crate::storage::save(&file,doc).and_then(|_|crate::storage::read(&file)).is_ok_and(|r|r.0==*doc))},
96=>("Transferência: undo restaura documento e estado salvo exatos",doc==&app.saved&&self.navigation.as_ref().is_some_and(|(d,h)|doc==d&&app.undo.len()==*h)),
97=>("Disponibilidade: controles de trás desabilitados no limite inferior",self.order_enabled[0]&&!self.order_enabled[1]&&self.order_enabled[4]&&!self.order_enabled[5]),
98=>("Disponibilidade: frente/subir desabilitados no limite superior",!self.order_enabled[0]&&self.order_enabled[1]&&!self.order_enabled[4]&&self.order_enabled[5]),
89=>("Ordem intercalada: fixture abre sem normalizar nem dirty",doc.objects.iter().map(|o|o.id).collect::<Vec<_>>()==[1,4,2,5,3]&&doc==&app.saved),
90=>("Ordem intercalada: passo conserva slots alheios e atributos",doc.objects.iter().map(|o|o.id).collect::<Vec<_>>()==[2,4,1,5,3]&&self.navigation.as_ref().is_some_and(|(d,h)|app.undo.len()==h+1&&doc.objects.iter().all(|o|d.objects.iter().any(|v|v==o))&&doc.pages[1]==d.pages[1])),
91=>("Ordem intercalada: comando inverso restaura conteúdo e estado salvo",self.navigation.as_ref().is_some_and(|(d,h)|doc==d&&doc==&app.saved&&app.undo.len()==h+2)),
92=>("Ordem intercalada: extrema da camada conserva slots alheios",doc.objects.iter().map(|o|o.id).collect::<Vec<_>>()==[2,4,3,5,1]&&self.navigation.as_ref().is_some_and(|(d,h)|app.undo.len()==h+1&&doc.objects.iter().all(|o|d.objects.iter().any(|v|v==o))&&doc.layers==d.layers)),
93=>{let file=app.smoke.as_ref().unwrap().with_extension("interleaved.artdraw");("Ordem intercalada: salvar e reabrir preserva representação exata",crate::storage::save(&file,doc).and_then(|_|crate::storage::read(&file)).is_ok_and(|r|r.0==*doc))},
73=>("Ordem: fixture de3 camadas/2 páginas aberta",doc.pages.len()==2&&doc.active_page==1&&doc.layers.len()==3&&paint_ids==[1,2,3,4,5]),
74=>("Ordem: CtrlPgUp avança uma posição",doc.active_page==1&&paint_ids==[2,1,3,4,5]&&app.selection.is_some_and(|i|doc.objects[i].id==1)),
75=>("Ordem: frente da camada preserva página/camadas",doc.active_page==1&&doc.active_layer==1&&paint_ids==[2,3,1,4,5]),
76=>("Ordem: passo atravessa limite e pula camada bloqueada",doc.active_page==1&&doc.active_layer==3&&paint_ids==[2,3,4,1,5]&&doc.objects.iter().find(|o|o.id==1).is_some_and(|o|o.layer==3)),
77=>("Ordem: trás da página/camada preserva ordem relativa",doc.active_page==1&&doc.active_layer==1&&paint_ids==[1,2,3,4,5]),
78=>("Ordem: frente da página usa camada editável superior",doc.active_page==1&&doc.active_layer==3&&paint_ids==[2,3,4,5,1]),
80=>{let path=app.smoke.as_ref().unwrap().with_extension("order.artdraw");("Ordem: salvar/reabrir documento e camadas",crate::storage::save(&path,doc).and_then(|_|crate::storage::read(&path)).is_ok_and(|r|r.0==*doc))},
81=>("Ordem: limite neutro não acrescenta histórico",self.navigation.as_ref().is_some_and(|(d,h)|d==doc&&*h==app.undo.len())),
82=>("Ordem: undo integral restaura fixture salva",doc==&app.saved&&paint_ids==[1,2,3,4,5]&&doc.active_page==1),
66=>("Folha: Ctrl+arrasto coalescido move apenas um membro",app.selected_indices()==vec![27]&&app.isolated_object==Some(doc.objects[27].id)&&self.navigation.as_ref().is_some_and(|(d,_)|doc.group_parents==d.group_parents&&doc.objects.iter().enumerate().all(|(i,o)|if i==27{near(o.bounds().x0,d.objects[i].bounds().x0+20.)&&near(o.bounds().y0,d.objects[i].bounds().y0+20.)}else{o==&d.objects[i]}))),
67=>("Folha: remover preenchimento preserva irmãos",app.selected_indices()==vec![27]&&doc.objects[27].fill.is_none()&&self.navigation.as_ref().is_some_and(|(d,_)|doc.objects[..27]==d.objects[..27]&&doc.group_parents==d.group_parents)),
68=>("Folha: copiar/colar cria objeto avulso",doc.objects.len()==29&&doc.objects[28].group.is_none()&&app.selected_indices()==vec![28]&&doc.group_parents.len()==2&&doc.objects[28].id!=doc.objects[27].id),
69=>("Folha: excluir membro preserva árvore e irmãos",doc.objects.len()==27&&doc.validate_groups().is_ok()&&self.navigation.as_ref().is_some_and(|(d,_)|doc.objects==d.objects[..27]&&doc.group_parents==d.group_parents)),
71=>("Folha: F10 converte só o retângulo interno",app.selected_indices()==vec![27]&&matches!(doc.objects[27].geometry,Geometry::Path{..})&&self.navigation.as_ref().is_some_and(|(d,_)|doc.objects[..27]==d.objects[..27]&&doc.group_parents==d.group_parents&&doc.objects[27].transform==d.objects[27].transform)),
72=>("Folha: arrastar nó interno preserva irmãos e hierarquia",app.selected_indices()==vec![27]&&self.navigation.as_ref().is_some_and(|(d,_)|doc.objects[..27]==d.objects[..27]&&doc.group_parents==d.group_parents&&matches!((&doc.objects[27].geometry,&d.objects[27].geometry),(Geometry::Path{nodes,..},Geometry::Path{nodes:old,..}) if nodes.len()==4&&old.len()==4&&nodes[1..]==old[1..]&&near((doc.objects[27].transform*nodes[0].p).x,(d.objects[27].transform*old[0].p).x+10.)&&near((doc.objects[27].transform*nodes[0].p).y,(d.objects[27].transform*old[0].p).y+10.)))),
70=>("Folha: undo restaura documento sem escopo obsoleto",app.isolated_object.is_none()&&self.navigation.as_ref().is_some_and(|(d,_)|doc==d)),
65=>("Grupos: seleção da cópia contém14 descendentes",app.selected_indices().len()==14&&app.selected_indices().iter().all(|&i|i>=14)),
64=>("Grupos: excluir árvore e undo conservam origem",doc.objects.len()==14&&doc.group_parents.len()==1&&doc.validate_groups().is_ok()),
56=>{let path=app.smoke.as_ref().unwrap().with_extension("multipage.artdraw");let result=crate::storage::save(&path,doc).and_then(|_|crate::storage::read(&path).map(|r|r.0));("Páginas: salvar/reler arquivo nativo completo",result.is_ok_and(|d|d==*doc))},
45=>("Teclado: Ctrl+Z rápido desfaz sem trocar ferramenta",doc.objects.is_empty()&&app.tool==Tool::Rectangle),
44=>("Mouse: arrasto completo no mesmo frame",doc.objects.len()==1&&near(doc.objects[0].bounds().width(),100.)&&near(doc.objects[0].bounds().height(),50.)),
42=>("Soltar arquivo: abre documento nativo",*doc==Document::demo()&&app.doc==app.saved&&app.file.as_ref().is_some_and(|p|p.extension().is_some_and(|e|e=="artdraw"))),
40=>("Recuperação: botão restaura documento editável",*doc==Document::demo()&&app.file.is_none()&&app.doc!=app.saved&&!app.recovery_dialog),
38=>("ArtDraw: round-trip nativo sem perda",crate::storage::decode(&crate::storage::encode(doc)).is_ok_and(|d|d==*doc)),
39=>("IDs: objetos copiados têm identidade única",doc.objects.iter().map(|o|o.id).collect::<std::collections::BTreeSet<_>>().len()==doc.objects.len()&&doc.objects.iter().all(|o|o.id>0)),
29=>{let result=svg::import(&svg::export(doc));("SVG: manter camadas/grupos/visibilidade",result.is_ok_and(|(d,_)|d==*doc))},
            _=>{let result=svg::import(&svg::export(doc));("SVG: round-trip do documento editado",result.is_ok_and(|(d,_)|d.objects.len()==doc.objects.len()))}
        };
        if !ok {
            if (66..=72).contains(&n) {
                self.results.push(format!(
                    "DIAGNÓSTICO folha: seleção={:?}, isolado={:?}, objetos={}, arestas={}",
                    app.selected_indices(),
                    app.isolated_object,
                    doc.objects.len(),
                    doc.group_parents.len()
                ));
            }
            if n == 23 || n == 24 {
                self.results.push(format!(
                    "DIAGNÓSTICO: camadas={:?}, ativa={}, objetos={}",
                    doc.layers,
                    doc.active_layer,
                    doc.objects.len()
                ));
            }
            self.failures += 1;
        }
        self.results
            .push(format!("{}: {label}", if ok { "OK" } else { "ERRO" }));
    }
}
fn button(raw: &mut egui::RawInput, pos: Pos2, button: PointerButton, pressed: bool) {
    raw.events.push(egui::Event::PointerMoved(pos));
    raw.events.push(egui::Event::PointerButton {
        pos,
        button,
        pressed,
        modifiers: raw.modifiers,
    });
}
