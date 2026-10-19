use super::*;
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ThemeKind {
    Light,
    Medium,
    Dark,
    Black,
}
impl ThemeKind {
    pub const ALL: [Self; 4] = [Self::Light, Self::Medium, Self::Dark, Self::Black];
    pub fn label(self) -> &'static str {
        match self {
            Self::Light => "Claro",
            Self::Medium => "Cinza",
            Self::Dark => "Escuro",
            Self::Black => "Preto",
        }
    }
    fn id(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Medium => "medium",
            Self::Dark => "dark",
            Self::Black => "black",
        }
    }
    fn path() -> Option<PathBuf> {
        std::env::var_os("ARTDRAW_SETTINGS_PATH")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("LOCALAPPDATA")
                    .map(|p| PathBuf::from(p).join("ArtDraw/settings.txt"))
            })
    }
    pub fn load() -> Self {
        Self::path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| Self::ALL.into_iter().find(|t| t.id() == s.trim()))
            .unwrap_or(Self::Dark)
    }
    pub fn save(self) {
        if let Some(p) = Self::path() {
            if let Some(parent) = p.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(p, self.id());
        }
    }
    pub fn chrome(self) -> Color32 {
        match self {
            Self::Light => Color32::from_rgb(229, 232, 237),
            Self::Medium => Color32::from_rgb(65, 69, 78),
            Self::Dark => Color32::from_rgb(29, 32, 38),
            Self::Black => Color32::from_rgb(13, 15, 19),
        }
    }
    pub fn panel(self) -> Color32 {
        match self {
            Self::Light => Color32::from_rgb(245, 246, 249),
            Self::Medium => Color32::from_rgb(79, 83, 93),
            Self::Dark => Color32::from_rgb(43, 47, 55),
            Self::Black => Color32::from_rgb(24, 27, 32),
        }
    }
    pub fn canvas(self) -> Color {
        match self {
            Self::Light => [184, 188, 197, 255],
            Self::Medium => [110, 114, 124, 255],
            Self::Dark => [77, 81, 90, 255],
            Self::Black => [45, 48, 55, 255],
        }
    }
    pub fn apply(self, ctx: &egui::Context) {
        ctx.set_theme(if self == Self::Light {
            egui::ThemePreference::Light
        } else {
            egui::ThemePreference::Dark
        });
        let mut v = if self == Self::Light {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        };
        v.panel_fill = self.panel();
        v.window_fill = self.panel();
        v.selection.bg_fill = Color32::from_rgb(44, 85, 139);
        v.selection.stroke = Stroke::new(1.0f32, ACCENT);
        ctx.set_visuals(v);
    }
}
