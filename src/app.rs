use crate::ordering::{self, Order};
use crate::{
    document::{Color, Document, Geometry, Node, Object, BLUE, INK},
    render::CanvasRenderer,
    svg,
};
use eframe::egui::{
    self, Align2, Color32, FontId, Key, PointerButton, Pos2, Rect as ERect, Sense, Stroke,
    Vec2 as EVec2,
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use vello::kurbo::{Point, Rect, Vec2};
mod interaction;
mod relative;
use relative::RelativePick;
mod pages;
mod professional;
mod theme;
use crate::editing;
use std::collections::BTreeSet;
use theme::ThemeKind;
mod validation;
use validation::Validation;

const ACCENT: Color32 = Color32::from_rgb(89, 158, 255);
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tool {
    Select,
    Shape,
    Rectangle,
    Ellipse,
    Pen,
    Text,
    Hand,
    Polygon,
    Star,
    Spiral,
    Line,
    Freehand,
}
impl Tool {
    fn label(self) -> &'static str {
        match self {
            Self::Select => "Seleção",
            Self::Shape => "Forma",
            Self::Rectangle => "Retângulo",
            Self::Ellipse => "Elipse",
            Self::Pen => "Caneta",
            Self::Text => "Texto",
            Self::Hand => "Mão",
            Self::Polygon => "Polígono",
            Self::Star => "Estrela",
            Self::Spiral => "Espiral",
            Self::Line => "Linha",
            Self::Freehand => "Mão livre",
        }
    }
    fn shortcut(self) -> &'static str {
        match self {
            Self::Select => "V / Espaço",
            Self::Shape => "F10",
            Self::Rectangle => "F6 / R",
            Self::Ellipse => "F7 / E",
            Self::Pen => "P",
            Self::Text => "F8 / T",
            Self::Hand => "H",
            Self::Polygon => "Y",
            Self::Star => "S",
            Self::Spiral => "I",
            Self::Line => "L",
            Self::Freehand => "F5",
        }
    }
    fn hint(self) -> &'static str {
        match self {Self::Select=>"Arraste para mover · Alças para redimensionar · Shift mantém proporção",Self::Shape=>"Arraste nós ou alças · Duplo clique na curva insere um nó · Delete remove nó",Self::Rectangle|Self::Ellipse=>"Arraste para desenhar · Shift mantém proporção",Self::Pen=>"Clique: linha · Clique e arraste: curva · Enter: terminar · Primeiro nó: fechar · Esc: cancelar",Self::Text=>"Clique na página para inserir texto · Duplo clique no texto para editar",Self::Hand=>"Arraste para navegar · Ctrl + roda: zoom",Self::Polygon|Self::Star|Self::Spiral=>"Arraste para desenhar · Ajuste lados/voltas na barra · Shift mantém proporção",Self::Line=>"Arraste para criar uma linha · Shift restringe a 45°",Self::Freehand=>"Arraste para desenhar livremente"}
    }
}
#[derive(Clone)]
enum Drag {
    Draw {
        start: Point,
    },
    Move {
        start: Point,
        originals: Vec<(usize, Object)>,
    },
    Resize {
        corner: usize,
        from: Rect,
        originals: Vec<(usize, Object)>,
    },
    Marquee {
        start: Point,
        current: Point,
        base: Vec<usize>,
    },
    Freehand {
        points: Vec<Point>,
    },
    Node {
        index: usize,
        node: usize,
        part: u8,
    },
    PenHandle {
        node: usize,
    },
    Pan {
        start: Pos2,
        original: EVec2,
    },
}
struct TextEditor {
    index: Option<usize>,
    position: Point,
    text: String,
    size: f64,
    focus: bool,
}
pub struct ArtDraw {
    startup_splash: Option<crate::branding::StartupSplash>,
    recovery: Option<crate::recovery::Recovery>,
    recoverable: Vec<crate::recovery::Candidate>,
    recovery_dialog: bool,
    doc: Document,
    page_manager: bool,
    page_name: String,
    pending_page_delete: Option<u64>,
    saved: Document,
    file: Option<PathBuf>,
    selection: Option<usize>,
    isolated_object: Option<u64>,
    additional: BTreeSet<usize>,
    clipboard: Vec<Object>,
    clipboard_parents: std::collections::BTreeMap<u64, u64>,
    theme: ThemeKind,
    shape_sides: usize,
    rotation_delta: f64,
    skew_delta: f64,
    align_page: bool,
    snap_grid: bool,
    grid_step: f64,
    selected_node: Option<usize>,
    pending_relative: Option<RelativePick>,
    relative_click_active: bool,
    tool: Tool,
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_width: f64,
    undo: Vec<Document>,
    redo: Vec<Document>,
    drag: Option<Drag>,
    drag_before: Option<Document>,
    pen: Vec<Node>,
    draft: Option<Object>,
    zoom: f64,
    pan: EVec2,
    fit: bool,
    grid: bool,
    renderer: CanvasRenderer,
    status: String,
    error: Option<String>,
    text_editor: Option<TextEditor>,
    about: bool,
    smoke: Option<PathBuf>,
    frames: u64,
    started: Instant,
    close_ready: bool,
    validation: Option<Validation>,
    capture_requested: bool,
}
impl ArtDraw {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        smoke: Option<PathBuf>,
        interactions: bool,
        startup_splash: Option<crate::branding::StartupSplash>,
        recovery_test: bool,
        pdf_preview: bool,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let theme = if smoke.is_some() {
            ThemeKind::Dark
        } else {
            ThemeKind::load()
        };
        cc.egui_ctx.set_theme(egui::ThemePreference::Dark);
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(37, 40, 47);
        visuals.window_fill = Color32::from_rgb(42, 46, 54);
        visuals.extreme_bg_color = Color32::from_rgb(26, 29, 35);
        visuals.faint_bg_color = Color32::from_rgb(47, 51, 60);
        visuals.selection.bg_fill = Color32::from_rgb(44, 85, 139);
        visuals.selection.stroke = Stroke::new(1.0f32, ACCENT);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(49, 54, 64);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(63, 73, 89);
        cc.egui_ctx.set_visuals(visuals);
        cc.egui_ctx.style_mut(|s| {
            s.spacing.item_spacing = egui::vec2(8., 7.);
            s.spacing.button_padding = egui::vec2(9., 6.);
            s.text_styles
                .insert(egui::TextStyle::Body, FontId::proportional(14.));
        });
        theme.apply(&cc.egui_ctx);
        let state = cc
            .wgpu_render_state
            .clone()
            .ok_or("O backend wgpu não está disponível")?;
        let renderer = CanvasRenderer::new(state)?;
        let doc = if pdf_preview {
            crate::pdf::proof_document()
        } else if smoke.is_some() && !interactions && !recovery_test {
            Document::demo()
        } else {
            Document::default()
        };
        if let Some(splash) = &startup_splash {
            splash.stage(2);
        }
        let recovery = if smoke.is_none() || recovery_test {
            crate::recovery::Recovery::new()
        } else {
            None
        };
        let recoverable = recovery
            .as_ref()
            .map(|r| r.candidates())
            .unwrap_or_default();
        let recovery_dialog = !recoverable.is_empty();
        Ok(Self {
            recovery,
            recoverable,
            recovery_dialog,
            startup_splash,
            page_manager: false,
            page_name: doc.name.clone(),
            pending_page_delete: None,
            saved: doc.clone(),
            doc,
            file: None,
            selection: None,
            isolated_object: None,
            additional: BTreeSet::new(),
            clipboard: Vec::new(),
            clipboard_parents: Default::default(),
            theme,
            shape_sides: 5,
            rotation_delta: 15.,
            skew_delta: 10.,
            align_page: false,
            snap_grid: false,
            grid_step: 20.,
            selected_node: None,
            pending_relative: None,
            relative_click_active: false,
            tool: Tool::Select,
            fill: Some(BLUE),
            stroke: Some(INK),
            stroke_width: 1.6,
            undo: Vec::new(),
            redo: Vec::new(),
            drag: None,
            drag_before: None,
            pen: Vec::new(),
            draft: None,
            zoom: 0.7,
            pan: EVec2::ZERO,
            fit: true,
            grid: false,
            renderer,
            status: "Pronto para criar".into(),
            error: None,
            text_editor: None,
            about: false,
            smoke,
            frames: 0,
            started: Instant::now(),
            close_ready: false,
            validation: if recovery_test {
                Some(Validation::recovery())
            } else {
                interactions.then(Validation::new)
            },
            capture_requested: false,
        })
    }
    fn remember(&mut self, before: Document) {
        if before != self.doc {
            self.cancel_relative_order();
            self.undo.push(before);
            if self.undo.len() > 100 {
                self.undo.remove(0);
            }
            self.redo.clear();
        }
    }
    fn undo(&mut self) {
        self.cancel_gesture();
        let view = (self.doc.active_page, self.doc.width, self.doc.height);
        if let Some(previous) = self.undo.pop() {
            self.redo.push(std::mem::replace(&mut self.doc, previous));
            self.selection = None;
            self.isolated_object = None;
            self.additional.clear();
            self.selected_node = None;
            self.page_name = self.doc.name.clone();
            if view != (self.doc.active_page, self.doc.width, self.doc.height) {
                self.reset_page_view();
            }
            self.status = "Desfeito".into();
        }
    }
    fn redo(&mut self) {
        self.cancel_gesture();
        let view = (self.doc.active_page, self.doc.width, self.doc.height);
        if let Some(next) = self.redo.pop() {
            self.undo.push(std::mem::replace(&mut self.doc, next));
            self.selection = None;
            self.isolated_object = None;
            self.additional.clear();
            self.selected_node = None;
            self.page_name = self.doc.name.clone();
            if view != (self.doc.active_page, self.doc.width, self.doc.height) {
                self.reset_page_view();
            }
            self.status = "Refeito".into();
        }
    }
    fn switch_tool(&mut self, tool: Tool) {
        self.cancel_relative_order();
        if !self.pen.is_empty() {
            self.finish_pen(false);
        }
        if let Some(before) = self.drag_before.take() {
            self.remember(before);
        }
        self.drag = None;
        self.draft = None;
        self.tool = tool;
        self.selected_node = None;
        if tool == Tool::Shape {
            self.make_editable();
        }
    }
    fn make_editable(&mut self) {
        if self.isolated_object.is_none() && self.selected_indices().len() > 1 {
            self.set_selection(Vec::new());
            self.status = "Selecione uma folha com Ctrl+clique ou no painel para editar nós".into();
            return;
        }
        if let Some(i) = self.selection {
            if !matches!(self.doc.objects[i].geometry, Geometry::Path { .. }) {
                let before = self.doc.clone();
                self.doc.objects[i].convert_to_path();
                self.remember(before);
                if !matches!(self.doc.objects[i].geometry, Geometry::Path { .. }) {
                    self.status =
                        "Este objeto tem vários contornos; mova e redimensione com Seleção".into();
                }
            }
        }
    }
    fn confirm_unsaved(&mut self) -> bool {
        if self.doc.content_eq(&self.saved) {
            return true;
        }
        match rfd::MessageDialog::new()
            .set_title("Salvar alterações?")
            .set_description("O documento contém alterações. Deseja salvá-las antes de continuar?")
            .set_buttons(rfd::MessageButtons::YesNoCancel)
            .show()
        {
            rfd::MessageDialogResult::Yes => self.save(false),
            rfd::MessageDialogResult::No => true,
            _ => false,
        }
    }
    fn new_document(&mut self, demo: bool) {
        if !self.confirm_unsaved() {
            return;
        }
        if let Some(r) = &mut self.recovery {
            r.clear_current();
        }
        self.cancel_gesture();
        self.doc = if demo {
            Document::demo()
        } else {
            Document::default()
        };
        self.saved = self.doc.clone();
        self.file = None;
        self.selection = None;
        self.isolated_object = None;
        self.additional.clear();
        self.undo.clear();
        self.redo.clear();
        self.fit = true;
        self.status = "Novo documento".into();
    }
    fn open(&mut self, insert: bool) {
        self.cancel_relative_order();
        if insert && !self.active_layer_editable() {
            self.status = "Ative uma camada editável para importar".into();
            return;
        }
        if !insert && !self.confirm_unsaved() {
            return;
        }
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Desenhos ArtDraw e SVG", &["artdraw", "svg"])
            .pick_file()
        {
            self.load_path(path, insert);
        }
    }
    fn load_path(&mut self, path: PathBuf, insert: bool) {
        match crate::storage::read(&path) {
            Ok((doc, warnings)) => {
                if insert && doc.pages.len() > 1 {
                    self.error=Some("Importar um documento multipágina nesta página não é suportado. Abra o documento completo ou exporte uma página como SVG.".into());
                    return;
                }
                self.cancel_gesture();
                if insert {
                    let before = self.doc.clone();
                    let ids =
                        editing::insert_objects(&mut self.doc, &doc.objects, &doc.group_parents);
                    self.set_selection(ids);
                    self.remember(before);
                } else {
                    if let Some(r) = &mut self.recovery {
                        r.clear_current();
                    }
                    self.doc = doc;
                    self.saved = self.doc.clone();
                    self.file = Some(path);
                    self.selection = None;
                    self.isolated_object = None;
                    self.additional.clear();
                    self.undo.clear();
                    self.redo.clear();
                    self.fit = true;
                }
                self.status = format!(
                    "Documento carregado · {} página(s) · {} objetos",
                    self.doc.pages.len(),
                    self.doc.total_objects()
                );
                if !warnings.is_empty() {
                    self.error = Some(format!(
                        "O arquivo foi aberto com estas adaptações:\n\n{}",
                        warnings.join("\n")
                    ));
                }
            }
            Err(e) => self.error = Some(e),
        }
    }
    fn save(&mut self, save_as: bool) -> bool {
        self.cancel_relative_order();
        if !self.pen.is_empty() {
            self.finish_pen(false);
        }
        let path = if save_as || self.file.is_none() {
            rfd::FileDialog::new()
                .add_filter("Documento ArtDraw", &["artdraw"])
                .add_filter("Desenho SVG", &["svg"])
                .set_file_name(
                    self.file
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .and_then(|s| s.to_str())
                        .unwrap_or("Desenho.artdraw"),
                )
                .save_file()
        } else {
            self.file.clone()
        };
        let Some(mut path) = path else {
            return false;
        };
        if path.extension().is_none() {
            path.set_extension("artdraw");
        }
        self.save_to_path(path)
    }
    fn save_to_path(&mut self, path: PathBuf) -> bool {
        match crate::storage::save(&path, &self.doc) {
            Ok(()) => {
                self.file = Some(path);
                self.saved = self.doc.clone();
                if let Some(r) = &mut self.recovery {
                    r.saved();
                }
                self.status = "Documento salvo".into();
                true
            }
            Err(e) => {
                self.error = Some(format!("Não foi possível salvar: {e}"));
                false
            }
        }
    }
    fn recovery_ui(&mut self, ctx: &egui::Context) {
        if !self.recovery_dialog {
            return;
        }
        let mut pick = None;
        let mut later = false;
        egui::Window::new("Recuperação de documentos")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .show(ctx, |ui| {
                ui.label("Foram encontrados pontos de recuperação de sessões encerradas.");
                if self.recoverable.is_empty() {
                    ui.label("Nenhum documento pendente de recuperação.");
                }
                for (i, c) in self.recoverable.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{} página(s) · {} objetos · página ativa {} × {} px",
                            c.document.pages.len(),
                            c.document.total_objects(),
                            c.document.width,
                            c.document.height
                        ));
                        let recover = ui.button("Recuperar");
                        if let Some(v) = &mut self.validation {
                            v.widgets.insert("recover", recover.rect.center());
                        }
                        if recover.clicked() {
                            pick = Some(i);
                        }
                    });
                }
                ui.separator();
                if ui.button("Continuar sem recuperar agora").clicked() {
                    later = true;
                }
            });
        if let Some(i) = pick {
            if self.confirm_unsaved() {
                let candidate = self.recoverable.remove(i);
                self.cancel_gesture();
                self.doc = candidate.document;
                self.saved = Document::default();
                self.file = None;
                self.selection = None;
                self.isolated_object = None;
                self.additional.clear();
                self.undo.clear();
                self.redo.clear();
                self.fit = true;
                if let Some(r) = &mut self.recovery {
                    r.chose(candidate.path);
                }
                self.status = "Documento recuperado; salve para manter as alterações".into();
                self.recovery_dialog = false;
            }
        }
        if later {
            self.recovery_dialog = false;
        }
    }
    fn export_svg(&mut self) {
        if let Some(mut path) = rfd::FileDialog::new()
            .add_filter("Desenho SVG", &["svg"])
            .set_title("Exportar página ativa como SVG")
            .set_file_name("Pagina.svg")
            .save_file()
        {
            if path.extension().is_none() {
                path.set_extension("svg");
            }
            if !path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("svg"))
            {
                self.error = Some("Use a extensão .svg para exportar".into());
                return;
            }
            match crate::storage::atomic_write(&path, svg::export(&self.doc).as_bytes(), true) {
                Ok(()) => {
                    self.status = format!(
                        "Página ‘{}’ exportada como SVG; {} página(s) mantidas no documento",
                        self.doc.name,
                        self.doc.pages.len()
                    )
                }
                Err(e) => self.error = Some(format!("Falha ao exportar: {e}")),
            }
        }
    }
    fn export_pdf(&mut self, all_pages: bool) {
        let title = if all_pages {
            "Exportar documento PDF - RGB, texto em curvas"
        } else {
            "Exportar página PDF - RGB, texto em curvas"
        };
        let name = self
            .file
            .as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or("desenho");
        if let Some(file) = rfd::FileDialog::new()
            .set_title(title)
            .add_filter("PDF vetorial RGB (texto em curvas)", &["pdf"])
            .set_file_name(format!("{name}.pdf"))
            .save_file()
        {
            match crate::pdf::save(&file, &self.doc, all_pages) {
                Ok(()) => {
                    self.status = format!(
                        "PDF exportado: {} página(s), RGB e texto em curvas",
                        if all_pages { self.doc.pages.len() } else { 1 }
                    )
                }
                Err(e) => self.error = Some(format!("Falha ao exportar PDF: {e}")),
            }
        }
    }
    fn duplicate(&mut self) {
        if !self.active_layer_editable() {
            self.status = "Ative uma camada visível e desbloqueada".into();
            return;
        }
        let indices = self.selected_indices();
        let mut objects = indices
            .iter()
            .map(|&i| self.doc.objects[i].clone())
            .collect::<Vec<_>>();
        let before = self.doc.clone();
        let mut parents = self.doc.group_parents.clone();
        if self.isolated_object.is_some() {
            for object in &mut objects {
                object.group = None;
            }
            parents.clear();
        }
        let ids = editing::duplicate(&mut self.doc, &objects, &parents, Vec2::new(16., 16.));
        self.set_selection(ids);
        self.remember(before);
    }
    fn delete(&mut self) {
        if let Some(i) = self.selection {
            let before = self.doc.clone();
            let mut removed_node = false;
            if self.tool == Tool::Shape {
                if let Some(n) = self.selected_node {
                    if let Geometry::Path { nodes, closed } = &mut self.doc.objects[i].geometry {
                        if nodes.len() > 2 {
                            nodes.remove(n);
                            self.selected_node = None;
                            removed_node = true;
                        } else {
                            *closed = false;
                        }
                    }
                }
            }
            if !removed_node {
                let selected: BTreeSet<_> = self.selected_indices().into_iter().collect();
                self.doc.objects = std::mem::take(&mut self.doc.objects)
                    .into_iter()
                    .enumerate()
                    .filter(|(index, _)| !selected.contains(index))
                    .map(|(_, o)| o)
                    .collect();
                self.doc.prune_groups();
                self.selection = None;
                self.isolated_object = None;
                self.additional.clear();
                self.selected_node = None;
            }
            self.remember(before);
        }
    }
    fn apply_color(&mut self, color: Option<Color>, stroke: bool) {
        if stroke {
            self.stroke = color;
        } else {
            self.fill = color;
        }
        let indices = self.selected_indices();
        if !indices.is_empty() {
            let before = self.doc.clone();
            for i in indices {
                if stroke {
                    self.doc.objects[i].stroke = color;
                    if self.doc.objects[i].stroke_width == 0. {
                        self.doc.objects[i].stroke_width = self.stroke_width;
                    }
                } else {
                    self.doc.objects[i].fill = color;
                }
            }
            self.remember(before);
        }
    }
    fn shortcuts(&mut self, ctx: &egui::Context) {
        // Each event retains its modifiers even if Ctrl/Shift was released before redraw.
        let keys = ctx.input(|i| {
            i.events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => Some((*key, *modifiers)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        });
        for (key, modifiers) in keys {
            self.shortcut_key(ctx, key, modifiers);
        }
    }
    fn shortcut_key(&mut self, ctx: &egui::Context, key: Key, modifiers: egui::Modifiers) {
        if key == Key::Escape && self.pending_relative.is_some() { self.cancel_relative_order(); return; }
        let command = modifiers.command || modifiers.ctrl;
        let pressed = |candidate| key == candidate;
        if command {
            if pressed(Key::S) {
                self.save(modifiers.shift);
            } else if pressed(Key::O) {
                self.open(false);
            } else if pressed(Key::I) {
                self.open(true);
            } else if pressed(Key::N) {
                self.new_document(false);
            } else if !ctx.wants_keyboard_input() {
                if pressed(Key::Z) {
                    if modifiers.shift {
                        self.redo();
                    } else {
                        self.undo();
                    }
                }
                if pressed(Key::Y) {
                    self.redo();
                }
                if pressed(Key::A) {
                    let ids = editing::expand_groups(&self.doc, 0..self.doc.objects.len());
                    self.set_selection(ids);
                }
                if pressed(Key::C) {
                    self.copy_selection();
                }
                if pressed(Key::X) {
                    self.copy_selection();
                    self.delete();
                }
                if pressed(Key::V) {
                    self.paste_selection();
                }
                if pressed(Key::U) && !modifiers.shift {
                    self.ungroup_selection();
                }
                if pressed(Key::G) {
                    if modifiers.shift {
                        self.ungroup_selection();
                    } else {
                        self.group_selection();
                    }
                }
                if pressed(Key::PageUp) {
                    self.order_command(Order::ForwardOne);
                }
                if pressed(Key::PageDown) {
                    self.order_command(Order::BackOne);
                }
                if pressed(Key::Home) {
                    self.order_selection(true);
                }
                if pressed(Key::End) {
                    self.order_selection(false);
                }
                if pressed(Key::D) {
                    self.duplicate();
                }
            }
            return;
        }
        if ctx.wants_keyboard_input() {
            return;
        }
        if modifiers.shift && pressed(Key::PageUp) {
            self.order_command(Order::FrontLayer);
            return;
        }
        if modifiers.shift && pressed(Key::PageDown) {
            self.order_command(Order::BackLayer);
            return;
        }
        if pressed(Key::PageUp) {
            self.relative_page(-1);
            return;
        }
        if pressed(Key::PageDown) {
            self.relative_page(1);
            return;
        }
        if pressed(Key::Escape) {
            self.cancel_gesture();
            self.selection = None;
            self.isolated_object = None;
            self.additional.clear();
            self.selected_node = None;
        }
        if pressed(Key::Enter) && self.tool == Tool::Pen {
            self.finish_pen(false);
        }
        if pressed(Key::Delete) || pressed(Key::Backspace) {
            self.delete();
        }
        for (key, tool) in [
            (Key::V, Tool::Select),
            (Key::F10, Tool::Shape),
            (Key::R, Tool::Rectangle),
            (Key::F6, Tool::Rectangle),
            (Key::E, Tool::Ellipse),
            (Key::F7, Tool::Ellipse),
            (Key::P, Tool::Pen),
            (Key::T, Tool::Text),
            (Key::F8, Tool::Text),
            (Key::H, Tool::Hand),
            (Key::Y, Tool::Polygon),
            (Key::S, Tool::Star),
            (Key::I, Tool::Spiral),
            (Key::L, Tool::Line),
            (Key::F5, Tool::Freehand),
        ] {
            if pressed(key) {
                self.switch_tool(tool);
            }
        }
        if pressed(Key::F4) {
            self.fit = true;
        }
        let step = if modifiers.shift { 10. } else { 1. };
        let d = Vec2::new(
            if pressed(Key::ArrowRight) {
                step
            } else if pressed(Key::ArrowLeft) {
                -step
            } else {
                0.
            },
            if pressed(Key::ArrowDown) {
                step
            } else if pressed(Key::ArrowUp) {
                -step
            } else {
                0.
            },
        );
        if d.hypot2() > 0. {
            if let Some(i) = self.selection {
                let before = self.doc.clone();
                let _ = i;
                let ids = self.selected_indices();
                editing::transform(&mut self.doc, &ids, vello::kurbo::Affine::translate(d));
                self.remember(before);
            }
        }
    }
    fn menus(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu")
            .frame(
                egui::Frame::new()
                    .fill(self.theme.chrome())
                    .inner_margin(egui::Margin::symmetric(12, 7)),
            )
            .show(ctx, |ui| {
                if self.text_editor.is_some() {
                    ui.disable();
                }
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("A").size(23.).strong().color(ACCENT));
                    ui.label(egui::RichText::new("ArtDraw").strong().size(18.));
                    ui.add_space(15.);
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("Arquivo", |ui| {
                            if ui.button("Novo                         Ctrl+N").clicked() {
                                self.new_document(false);
                                ui.close_menu();
                            }
                            if ui.button("Abrir ArtDraw / SVG…   Ctrl+O").clicked() {
                                self.open(false);
                                ui.close_menu();
                            }
                            if ui.button("Importar desenho…       Ctrl+I").clicked() {
                                self.open(true);
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui
                                .button("Salvar                           Ctrl+S")
                                .clicked()
                            {
                                self.save(false);
                                ui.close_menu();
                            }
                            if ui.button("Salvar como…    Ctrl+Shift+S").clicked() {
                                self.save(true);
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button("Exportar página ativa como SVG…").clicked() {
                                self.export_svg();
                                ui.close_menu();
                            }
                            if ui.button("Exportar documento como PDF…").on_hover_text("Todas as páginas; RGB, texto convertido em curvas; sem ICC/CMYK/PDF-X").clicked(){self.export_pdf(true);ui.close_menu();}
                            if ui.button("Exportar página ativa como PDF…").on_hover_text("Página atual; RGB, texto convertido em curvas").clicked(){self.export_pdf(false);ui.close_menu();}
                            if ui.button("Abrir exemplo").clicked() {
                                self.new_document(true);
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button("Sair").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                ui.close_menu();
                            }
                        });
                        ui.menu_button("Editar", |ui| {
                            if ui
                                .add_enabled(
                                    !self.undo.is_empty(),
                                    egui::Button::new("Desfazer          Ctrl+Z"),
                                )
                                .clicked()
                            {
                                self.undo();
                                ui.close_menu();
                            }
                            if ui
                                .add_enabled(
                                    !self.redo.is_empty(),
                                    egui::Button::new("Refazer            Ctrl+Y"),
                                )
                                .clicked()
                            {
                                self.redo();
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui
                                .add_enabled(
                                    self.selection.is_some(),
                                    egui::Button::new("Duplicar           Ctrl+D"),
                                )
                                .clicked()
                            {
                                self.duplicate();
                                ui.close_menu();
                            }
                            if ui
                                .add_enabled(
                                    self.selection.is_some(),
                                    egui::Button::new("Excluir              Delete"),
                                )
                                .clicked()
                            {
                                self.delete();
                                ui.close_menu();
                            }
                        });
                        self.object_menu(ui);
                        let theme_menu = ui.menu_button("Exibir", |ui| {
                            if ui.button("Ajustar página     F4").clicked() {
                                self.fit = true;
                                ui.close_menu();
                            }
                            if ui.button("Zoom 100%").clicked() {
                                self.zoom = 1.;
                                self.pan = EVec2::ZERO;
                                self.fit = false;
                                ui.close_menu();
                            }
                            ui.checkbox(&mut self.grid, "Grade de referência");
                            ui.checkbox(&mut self.snap_grid, "Ajustar à grade");
                            ui.add(
                                egui::DragValue::new(&mut self.grid_step)
                                    .range(1.0..=500.0)
                                    .prefix("Passo "),
                            );
                            ui.separator();
                            ui.label("Tema");
                            for kind in ThemeKind::ALL {
                                let response =
                                    ui.selectable_label(self.theme == kind, kind.label());
                                if let Some(v) = self.validation.as_mut() {
                                    v.widgets.insert(
                                        if kind == ThemeKind::Light {
                                            "theme-light"
                                        } else if kind == ThemeKind::Dark {
                                            "theme-dark"
                                        } else {
                                            "theme-other"
                                        },
                                        response.rect.center(),
                                    );
                                }
                                if response.clicked() {
                                    self.theme = kind;
                                    kind.apply(ctx);
                                    if self.smoke.is_none() {
                                        kind.save();
                                    }
                                    ui.close_menu();
                                }
                            }
                        });
                        if let Some(v) = self.validation.as_mut() {
                            v.widgets
                                .insert("theme-menu", theme_menu.response.rect.center());
                        }
                        ui.menu_button("Ajuda", |ui| {
                            if ui.button("Recuperar documentos…").clicked() {
                                self.recoverable = self
                                    .recovery
                                    .as_ref()
                                    .map(|r| r.candidates())
                                    .unwrap_or_default();
                                self.recovery_dialog = true;
                                ui.close_menu();
                            }
                            if ui.button("Atalhos e sobre o ArtDraw").clicked() {
                                self.about = true;
                                ui.close_menu();
                            }
                        });
                    });
                });
            });
        egui::TopBottomPanel::top("actions")
            .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(12, 7)))
            .show(ctx, |ui| {
                if self.text_editor.is_some() {
                    ui.disable();
                }
                ui.horizontal(|ui| {
                    if ui.button("Novo").clicked() {
                        self.new_document(false);
                    }
                    if ui.button("Abrir").clicked() {
                        self.open(false);
                    }
                    if ui.button("Salvar").clicked() {
                        self.save(false);
                    }
                    ui.separator();
                    if ui
                        .add_enabled(!self.undo.is_empty(), egui::Button::new("Desfazer"))
                        .clicked()
                    {
                        self.undo();
                    }
                    if ui
                        .add_enabled(!self.redo.is_empty(), egui::Button::new("Refazer"))
                        .clicked()
                    {
                        self.redo();
                    }
                    ui.separator();
                    if ui.button("Ajustar página").clicked() {
                        self.fit = true;
                    }
                    let mut percent = (self.zoom * 100.) as f32;
                    if ui
                        .add(
                            egui::DragValue::new(&mut percent)
                                .range(5.0..=1600.0)
                                .suffix(" %")
                                .speed(1.),
                        )
                        .changed()
                    {
                        self.zoom = f64::from(percent) / 100.;
                        self.fit = false;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("OFFLINE  /  SVG")
                                .size(11.)
                                .color(Color32::from_gray(150)),
                        );
                    });
                });
            });
        egui::TopBottomPanel::top("properties")
            .frame(
                egui::Frame::new()
                    .fill(self.theme.panel())
                    .inner_margin(egui::Margin::symmetric(12, 8)),
            )
            .show(ctx, |ui| {
                if self.text_editor.is_some() {
                    ui.disable();
                }
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(self.tool.label())
                            .color(ACCENT)
                            .strong(),
                    );
                    ui.separator();
                    let before = self.doc.clone();
                    if let Some(i) = self.selection {
                        let bounds = self
                            .selection_bounds()
                            .unwrap_or_else(|| self.doc.objects[i].bounds());
                        let (mut x, mut y, mut w, mut h) =
                            (bounds.x0, bounds.y0, bounds.width(), bounds.height());
                        let mut changed = false;
                        for (label, value) in
                            [("X", &mut x), ("Y", &mut y), ("L", &mut w), ("A", &mut h)]
                        {
                            ui.label(label);
                            changed |= ui
                                .add(
                                    egui::DragValue::new(value)
                                        .speed(1.)
                                        .max_decimals(1)
                                        .range(-100000.0..=100000.0),
                                )
                                .changed();
                        }
                        if changed {
                            self.resize_selection(
                                bounds,
                                Rect::new(x, y, x + w.max(0.1), y + h.max(0.1)),
                            );
                        }
                        ui.separator();
                        ui.label("Contorno");
                        ui.add(
                            egui::DragValue::new(&mut self.doc.objects[i].stroke_width)
                                .range(0.0..=200.0)
                                .speed(0.2)
                                .suffix(" px"),
                        );
                        let width = self.doc.objects[i].stroke_width;
                        if before.objects[i].stroke_width != width {
                            for index in self.selected_indices() {
                                self.doc.objects[index].stroke_width = width;
                            }
                        }
                    } else {
                        ui.label("Página");
                        ui.add(
                            egui::DragValue::new(&mut self.doc.width)
                                .range(32.0..=100000.0)
                                .prefix("L ")
                                .suffix(" px"),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.doc.height)
                                .range(32.0..=100000.0)
                                .prefix("A ")
                                .suffix(" px"),
                        );
                        ui.separator();
                        ui.label("Contorno");
                        ui.add(
                            egui::DragValue::new(&mut self.stroke_width)
                                .range(0.0..=200.0)
                                .speed(0.2)
                                .suffix(" px"),
                        );
                    }
                    if matches!(self.tool, Tool::Polygon | Tool::Star | Tool::Spiral) {
                        ui.label(if self.tool == Tool::Spiral {
                            "Voltas"
                        } else {
                            "Lados/pontas"
                        });
                        ui.add(egui::DragValue::new(&mut self.shape_sides).range(3..=32));
                    }
                    self.remember(before);
                });
            });
    }
    fn palette(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("palette")
            .exact_width(36.)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(self.theme.chrome())
                    .inner_margin(egui::Margin::symmetric(5, 9)),
            )
            .show(ctx, |ui| {
                if self.text_editor.is_some() {
                    ui.disable();
                }
                let colors: [[u8; 3]; 30] = [
                    [255, 255, 255],
                    [225, 228, 233],
                    [175, 183, 197],
                    [112, 122, 140],
                    [63, 71, 86],
                    [0, 0, 0],
                    [99, 102, 241],
                    [139, 92, 246],
                    [192, 38, 211],
                    [236, 72, 153],
                    [244, 63, 94],
                    [239, 68, 68],
                    [249, 115, 22],
                    [251, 146, 60],
                    [245, 158, 11],
                    [250, 204, 21],
                    [234, 179, 8],
                    [132, 204, 22],
                    [34, 197, 94],
                    [16, 185, 129],
                    [20, 184, 166],
                    [6, 182, 212],
                    [14, 165, 233],
                    [59, 130, 246],
                    [37, 99, 235],
                    [30, 64, 175],
                    [127, 29, 29],
                    [120, 53, 15],
                    [20, 83, 45],
                    [49, 46, 129],
                ];
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let (r, response) =
                        ui.allocate_exact_size(egui::vec2(24., 22.), Sense::click());
                    ui.painter().rect_filled(r, 2., Color32::WHITE);
                    ui.painter().line_segment(
                        [r.left_bottom(), r.right_top()],
                        Stroke::new(2.0f32, Color32::RED),
                    );
                    let response = response
                        .on_hover_text("Sem cor · Esquerdo: preenchimento · Direito: contorno");
                    if response.clicked() {
                        self.apply_color(None, false);
                    }
                    if response.secondary_clicked() {
                        self.apply_color(None, true);
                    }
                    ui.add_space(4.);
                    for c in colors {
                        let (r, response) =
                            ui.allocate_exact_size(egui::vec2(24., 18.), Sense::click());
                        ui.painter()
                            .rect_filled(r, 1., Color32::from_rgb(c[0], c[1], c[2]));
                        if c == [239, 68, 68] {
                            if let Some(v) = self.validation.as_mut() {
                                v.palette = Some(r.center());
                            }
                        }
                        let response = response.on_hover_text(format!(
                            "#{:02X}{:02X}{:02X}\nEsquerdo: preencher\nDireito: contorno",
                            c[0], c[1], c[2]
                        ));
                        if response.clicked() {
                            self.apply_color(Some([c[0], c[1], c[2], 255]), false);
                        }
                        if response.secondary_clicked() {
                            self.apply_color(Some([c[0], c[1], c[2], 255]), true);
                        }
                    }
                });
            });
    }
    fn toolbar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("tools")
            .exact_width(62.)
            .resizable(false)
            .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(8, 12)))
            .show(ctx, |ui| {
                if self.text_editor.is_some() {
                    ui.disable();
                }
                egui::ScrollArea::vertical()
                    .id_salt("toolbox")
                    .show(ui, |ui| {
                        for tool in [
                            Tool::Select,
                            Tool::Shape,
                            Tool::Pen,
                            Tool::Freehand,
                            Tool::Line,
                            Tool::Rectangle,
                            Tool::Ellipse,
                            Tool::Polygon,
                            Tool::Star,
                            Tool::Spiral,
                            Tool::Text,
                            Tool::Hand,
                        ] {
                            let (r, response) =
                                ui.allocate_exact_size(egui::vec2(44., 40.), Sense::click());
                            if self.tool == tool || response.hovered() {
                                ui.painter().rect_filled(
                                    r,
                                    4.,
                                    if self.tool == tool {
                                        Color32::from_rgb(47, 78, 117)
                                    } else {
                                        self.theme.chrome()
                                    },
                                );
                            }
                            draw_icon(
                                ui.painter(),
                                r,
                                tool,
                                if self.tool == tool {
                                    ACCENT
                                } else {
                                    ui.visuals().text_color()
                                },
                            );
                            if response
                                .on_hover_text(format!("{} ({})", tool.label(), tool.shortcut()))
                                .clicked()
                            {
                                self.switch_tool(tool);
                            }
                        }
                    });
            });
    }
    fn edit_text(&mut self, index: usize) {
        if !self.doc.editable(index) {
            return;
        }
        self.select_individual(Some(index));
        if let Geometry::Text {
            position,
            text,
            size,
        } = &self.doc.objects[index].geometry
        {
            self.text_editor = Some(TextEditor {
                index: Some(index),
                position: *position,
                text: text.clone(),
                size: *size,
                focus: true,
            });
        }
    }
    fn dialogs(&mut self, ctx: &egui::Context) {
        if let Some(mut editor) = self.text_editor.take() {
            let mut open = true;
            let mut apply = false;
            let mut cancel = false;
            egui::Window::new("Texto artístico")
                .collapsible(false)
                .resizable(true)
                .default_width(430.)
                .anchor(Align2::CENTER_CENTER, EVec2::ZERO)
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.label("Conteúdo");
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut editor.text)
                            .desired_rows(5)
                            .desired_width(f32::INFINITY),
                    );
                    if editor.focus {
                        response.request_focus();
                        editor.focus = false;
                    }
                    ui.horizontal(|ui| {
                        ui.label("Tamanho");
                        ui.add(
                            egui::DragValue::new(&mut editor.size)
                                .range(2.0..=2000.0)
                                .suffix(" px"),
                        );
                        ui.label("Ubuntu Light");
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        let response = ui.button("Aplicar texto");
                        if let Some(v) = self.validation.as_mut() {
                            v.apply = Some(response.rect.center());
                        }
                        if response.clicked() {
                            apply = true;
                        }
                        if ui.button("Cancelar").clicked() {
                            cancel = true;
                        }
                    });
                });
            if apply && !editor.text.trim().is_empty() {
                let before = self.doc.clone();
                if let Some(i) = editor.index {
                    if let Geometry::Text { text, size, .. } = &mut self.doc.objects[i].geometry {
                        *text = editor.text;
                        *size = editor.size;
                    }
                } else {
                    self.doc.add_object(Object::new(
                        "Texto",
                        Geometry::Text {
                            position: editor.position,
                            text: editor.text,
                            size: editor.size,
                        },
                        self.fill.or(Some(INK)),
                        None,
                        0.,
                    ));
                    self.set_selection(vec![self.doc.objects.len() - 1]);
                }
                self.remember(before);
            } else if open && !cancel && !apply {
                self.text_editor = Some(editor);
            }
        }
        if let Some(message) = self.error.clone() {
            let mut open = true;
            egui::Window::new("Aviso do ArtDraw")
                .open(&mut open)
                .collapsible(false)
                .default_width(470.)
                .show(ctx, |ui| {
                    ui.label(message);
                    if ui.button("Entendido").clicked() {
                        self.error = None;
                    }
                });
            if !open {
                self.error = None;
            }
        }
        if self.about {
            egui::Window::new("Sobre o ArtDraw").open(&mut self.about).default_width(500.).show(ctx,|ui|{
            ui.heading("ArtDraw 1.9");ui.label("Editor vetorial offline · Rust + egui + Vello");ui.separator();
            for (a,b) in [("V","Seleção"),("F10","Forma: nós e alças"),("F6 / R","Retângulo"),("F7 / E","Elipse"),("P","Caneta: clique para linha, arraste para curva"),("F8 / T","Texto"),("Ctrl + Z / Y","Desfazer / refazer"),("Ctrl + D","Duplicar seleção"),("F4","Ajustar página"),("Ctrl + roda","Zoom no ponteiro"),("Espaço + arrasto / botão central","Navegar"),("Enter / clique no primeiro nó","Terminar / fechar caminho")]{ui.horizontal(|ui|{ui.label(egui::RichText::new(a).strong());ui.label(b);});}
            ui.separator();ui.label("Implementação independente clean-room. Não é afiliada à Corel. SVG suporta primitivas, caminhos e estilos inline; recursos avançados são informados na importação.");
        });
        }
    }
}
impl eframe::App for ArtDraw {
    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        if let Some(mut validation) = self.validation.take() {
            validation.inject(self, raw);
            self.validation = Some(validation);
        }
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.frames >= 2 {
            if let Some(splash) = self.startup_splash.take() {
                splash.finish();
            }
        } else if self.startup_splash.is_some() {
            ctx.request_repaint();
        }
        self.frames += 1;
        if ctx.input(|i| i.viewport().close_requested()) && !self.close_ready {
            if self.confirm_unsaved() {
                if let Some(r) = &mut self.recovery {
                    r.clear_current();
                }
                self.close_ready = true;
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            }
        }
        if self.text_editor.is_none()
            && self.error.is_none()
            && !self.recovery_dialog
            && !self.page_manager
            && self.pending_page_delete.is_none()
        {
            self.shortcuts(ctx);
        }
        if let Some(path) = ctx.input(|i| i.raw.dropped_files.iter().find_map(|f| f.path.clone())) {
            if self.confirm_unsaved() {
                self.load_path(path, false);
            }
        }
        self.menus(ctx);
        egui::TopBottomPanel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(self.theme.chrome())
                    .inner_margin(egui::Margin::symmetric(12, 7)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&self.status).size(12.));
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!(
                            "{} × {} px",
                            self.doc.width as i64, self.doc.height as i64
                        ))
                        .size(12.),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Vello · GPU").color(ACCENT).size(11.))
                            .on_hover_text(&self.renderer.adapter);
                        ui.separator();
                        ui.label(format!("{:.0}%", self.zoom * 100.));
                    });
                });
            });
        self.pages_bar(ctx);
        self.palette(ctx);
        self.inspector(ctx);
        self.toolbar(ctx);
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::from_rgb(48, 52, 61)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(12.);
                    ui.label(
                        egui::RichText::new(format!(
                            "{}{}",
                            self.file
                                .as_ref()
                                .and_then(|p| p.file_name())
                                .and_then(|s| s.to_str())
                                .unwrap_or("Sem título"),
                            if !self.doc.content_eq(&self.saved) {
                                " *"
                            } else {
                                ""
                            }
                        ))
                        .size(12.),
                    );
                    ui.label(
                        egui::RichText::new(format!("/ {}", self.doc.name))
                            .size(11.)
                            .color(Color32::from_gray(135)),
                    );
                });
                ui.separator();
                self.canvas(ui);
            });
        self.dialogs(ctx);
        self.recovery_ui(ctx);
        self.pages_dialog(ctx);
        if self.drag.is_none()
            && self.pen.is_empty()
            && self.text_editor.is_none()
            && !self.close_ready
        {
            if let Some(recovery) = &mut self.recovery {
                if let Some(error) = recovery.tick(&self.doc, !self.doc.content_eq(&self.saved)) {
                    self.status = format!("Falha ao criar recuperação: {error}");
                }
                ctx.request_repaint_after(Duration::from_secs(1));
            }
        }
        if let Some(mut validation) = self.validation.take() {
            validation.check(self);
            self.validation = Some(validation);
        }
        let name = self
            .file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("Sem título");
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
            "{}{} — ArtDraw",
            name,
            if !self.doc.content_eq(&self.saved) {
                " *"
            } else {
                ""
            }
        )));
        if let Some(path) = &self.smoke {
            let mut done = false;
            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Screenshot { image, .. } = event {
                        let bytes: Vec<u8> =
                            image.pixels.iter().flat_map(|p| p.to_array()).collect();
                        if image::save_buffer(
                            path,
                            &bytes,
                            image.size[0] as u32,
                            image.size[1] as u32,
                            image::ColorType::Rgba8,
                        )
                        .is_ok()
                        {
                            let _ = std::fs::write(
                                path.with_extension("txt"),
                                format!(
                                    "{}Adapter: {}\nVello GPU frames: {}\nWindow: {}x{}\n",
                                    self.validation
                                        .as_ref()
                                        .map_or_else(|| "PASS\n".into(), Validation::report),
                                    self.renderer.adapter,
                                    self.renderer.frames,
                                    image.size[0],
                                    image.size[1]
                                ),
                            );
                            done = true;
                        }
                    }
                }
            });
            if self.frames >= 5
                && !self.capture_requested
                && self.validation.as_ref().is_none_or(Validation::finished)
            {
                self.capture_requested = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
            if done {
                if let Some(validation) = &self.validation {
                    let _ = std::fs::write(path.with_extension("svg"), svg::export(&self.doc));
                    if !validation.passed() {
                        std::process::exit(2);
                    }
                }
                self.close_ready = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if self.started.elapsed() > Duration::from_secs(45) {
                self.close_ready = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ctx.request_repaint_after(Duration::from_millis(60));
        }
    }
}
fn draw_icon(p: &egui::Painter, r: ERect, tool: Tool, c: Color32) {
    let r = r.shrink(12.);
    let a = r.min;
    let b = r.max;
    let center = r.center();
    let stroke = Stroke::new(1.8f32, c);
    match tool {
        Tool::Select => {
            p.add(egui::Shape::convex_polygon(
                vec![
                    a,
                    Pos2::new(a.x + 7., b.y),
                    center,
                    Pos2::new(b.x, a.y + 7.),
                ],
                c,
                Stroke::NONE,
            ));
        }
        Tool::Rectangle => {
            p.rect_stroke(r, 1., stroke, egui::StrokeKind::Middle);
        }
        Tool::Ellipse => {
            p.circle_stroke(center, 10., stroke);
        }
        Tool::Text => {
            p.text(
                center,
                Align2::CENTER_CENTER,
                "T",
                FontId::proportional(27.),
                c,
            );
        }
        Tool::Shape => {
            p.line_segment([a, b], stroke);
            p.rect_filled(ERect::from_center_size(a, egui::vec2(6., 6.)), 0., c);
            p.rect_filled(ERect::from_center_size(b, egui::vec2(6., 6.)), 0., c);
            p.circle_stroke(center, 4., stroke);
        }
        Tool::Pen => {
            p.add(egui::Shape::closed_line(
                vec![
                    Pos2::new(center.x, a.y),
                    Pos2::new(b.x, center.y + 2.),
                    Pos2::new(center.x, b.y),
                    Pos2::new(a.x, center.y + 2.),
                ],
                stroke,
            ));
            p.line_segment([center, Pos2::new(center.x, b.y)], stroke);
            p.circle_filled(center, 2., c);
        }
        Tool::Polygon | Tool::Star | Tool::Spiral | Tool::Line | Tool::Freehand => {
            if tool == Tool::Line {
                p.line_segment([a, b], stroke);
            } else if tool == Tool::Freehand || tool == Tool::Spiral {
                p.text(
                    center,
                    Align2::CENTER_CENTER,
                    if tool == Tool::Spiral { "@" } else { "~" },
                    FontId::proportional(25.),
                    c,
                );
            } else {
                let count = if tool == Tool::Star { 10 } else { 5 };
                let pts = (0..count)
                    .map(|k| {
                        let angle = -std::f32::consts::FRAC_PI_2
                            + std::f32::consts::TAU * k as f32 / count as f32;
                        let radius = if tool == Tool::Star && k % 2 == 1 {
                            4.
                        } else {
                            10.
                        };
                        center + egui::vec2(angle.cos(), angle.sin()) * radius
                    })
                    .collect();
                p.add(egui::Shape::closed_line(pts, stroke));
            }
        }
        Tool::Hand => {
            p.text(
                center,
                Align2::CENTER_CENTER,
                "✥",
                FontId::proportional(25.),
                c,
            );
        }
    }
}
