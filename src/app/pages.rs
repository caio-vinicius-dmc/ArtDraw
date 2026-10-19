use super::*;

pub(super) enum PageAction {
    Switch(u64),
    Add(bool),
    Delete(u64),
    Move(isize),
    Rename(String),
    Landscape(bool),
}
impl ArtDraw {
    pub(super) fn reset_page_view(&mut self) {
        self.cancel_relative_order();
        self.selection = None;
        self.isolated_object = None;
        self.additional.clear();
        self.selected_node = None;
        self.page_name = self.doc.name.clone();
        self.fit = true;
        self.pan = EVec2::ZERO;
    }
    pub(super) fn relative_page(&mut self, delta: isize) {
        let index = self
            .doc
            .pages
            .iter()
            .position(|p| p.id == self.doc.active_page)
            .unwrap();
        let next = index as isize + delta;
        if next >= 0 && (next as usize) < self.doc.pages.len() {
            self.page_action(PageAction::Switch(self.doc.pages[next as usize].id));
        }
    }
    fn page_action(&mut self, action: PageAction) {
        if self.text_editor.is_some() {
            return;
        }
        if !self.pen.is_empty() {
            self.finish_pen(false);
        }
        self.cancel_gesture();
        let navigation = matches!(action, PageAction::Switch(_));
        let before = self.doc.clone();
        let result = match action {
            PageAction::Switch(id) => {
                if self.doc.activate_page(id) {
                    Ok(())
                } else {
                    Err("Página ausente".into())
                }
            }
            PageAction::Add(copy) => self.doc.add_page(copy).map(|_| ()),
            PageAction::Delete(id) => self.doc.delete_page(id),
            PageAction::Move(delta) => {
                self.doc.move_page(delta);
                Ok(())
            }
            PageAction::Rename(name) => {
                let name = name.trim();
                if name.is_empty() || name.chars().count() > 256 {
                    Err("Use um nome entre1 e256 caracteres".into())
                } else {
                    self.doc.name = name.into();
                    Ok(())
                }
            }
            PageAction::Landscape(landscape) => {
                let (w, h) = (self.doc.width, self.doc.height);
                if (landscape && w < h) || (!landscape && w > h) {
                    self.doc.width = h;
                    self.doc.height = w;
                }
                Ok(())
            }
        };
        match result {
            Ok(()) => {
                if !navigation {
                    self.remember(before);
                }
                self.reset_page_view();
                self.status = format!(
                    "{} · {} página(s) · {} objetos no documento",
                    self.doc.name,
                    self.doc.pages.len(),
                    self.doc.total_objects()
                );
            }
            Err(e) => self.error = Some(e),
        }
    }
    pub(super) fn pages_bar(&mut self, ctx: &egui::Context) {
        let mut action = None;
        egui::TopBottomPanel::bottom("pages-bar").show(ctx, |ui| {
            let enabled = self.text_editor.is_none()
                && self.error.is_none()
                && !self.recovery_dialog
                && !self.page_manager
                && self.pending_page_delete.is_none();
            ui.add_enabled_ui(enabled, |ui| {
                ui.horizontal(|ui| {
                    let index = self
                        .doc
                        .pages
                        .iter()
                        .position(|p| p.id == self.doc.active_page)
                        .unwrap();
                    let prev = ui
                        .add_enabled(index > 0, egui::Button::new("◀"))
                        .on_hover_text("Página anterior · Page Up");
                    if let Some(v) = &mut self.validation {
                        v.widgets.insert("page-prev", prev.rect.center());
                    }
                    if prev.clicked() {
                        action = Some(PageAction::Switch(self.doc.pages[index - 1].id));
                    }
                    let next = ui
                        .add_enabled(index + 1 < self.doc.pages.len(), egui::Button::new("▶"))
                        .on_hover_text("Próxima página · Page Down");
                    if let Some(v) = &mut self.validation {
                        v.widgets.insert("page-next", next.rect.center());
                    }
                    if next.clicked() {
                        action = Some(PageAction::Switch(self.doc.pages[index + 1].id));
                    }
                    egui::ComboBox::from_id_salt("active-page")
                        .selected_text(format!(
                            "{} / {} · {}",
                            index + 1,
                            self.doc.pages.len(),
                            self.doc.name
                        ))
                        .show_ui(ui, |ui| {
                            for (i, p) in self.doc.pages.iter().enumerate() {
                                if ui
                                    .selectable_label(
                                        p.id == self.doc.active_page,
                                        format!("{} · {}", i + 1, p.name),
                                    )
                                    .clicked()
                                {
                                    action = Some(PageAction::Switch(p.id));
                                }
                            }
                        });
                    for (key, label, copy) in [
                        ("new-page", "+ Página", false),
                        ("duplicate-page", "Duplicar", true),
                    ] {
                        let response =
                            ui.add_enabled(self.doc.pages.len() < 1000, egui::Button::new(label));
                        if let Some(v) = &mut self.validation {
                            v.widgets.insert(key, response.rect.center());
                        }
                        if response.clicked() {
                            action = Some(PageAction::Add(copy));
                        }
                    }
                    let manage = ui.button("Páginas…");
                    if let Some(v) = &mut self.validation {
                        v.widgets.insert("page-manager", manage.rect.center());
                    }
                    if manage.clicked() {
                        self.page_manager = true;
                        self.page_name = self.doc.name.clone();
                    }
                });
            });
        });
        if let Some(action) = action {
            self.page_action(action);
        }
    }
    pub(super) fn pages_dialog(&mut self, ctx: &egui::Context) {
        let mut action = None;
        if self.page_manager {
            let mut open = true;
            let mut close = false;
            egui::Window::new("Páginas do documento")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0., 0.])
                .show(ctx, |ui| {
                    if self.pending_page_delete.is_some() {
                        ui.disable();
                    }
                    egui::ScrollArea::vertical()
                        .max_height(160.)
                        .show(ui, |ui| {
                            for (i, p) in self.doc.pages.iter().enumerate() {
                                if ui
                                    .selectable_label(
                                        p.id == self.doc.active_page,
                                        format!(
                                            "{} · {} · {:.0} × {:.0} px · {} objetos",
                                            i + 1,
                                            p.name,
                                            p.width,
                                            p.height,
                                            p.objects.len()
                                        ),
                                    )
                                    .clicked()
                                {
                                    action = Some(PageAction::Switch(p.id));
                                }
                            }
                        });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Nome:");
                        let name = ui.text_edit_singleline(&mut self.page_name);
                        if let Some(v) = &mut self.validation {
                            v.widgets.insert("page-name", name.rect.center());
                        }
                        let apply = ui.button("Renomear");
                        if let Some(v) = &mut self.validation {
                            v.widgets.insert("page-rename", apply.rect.center());
                        }
                        if apply.clicked() {
                            action = Some(PageAction::Rename(self.page_name.clone()));
                        }
                    });
                    ui.label(
                        "A orientação altera a página ativa; os objetos mantêm suas coordenadas.",
                    );
                    ui.horizontal(|ui| {
                        for (key, label, landscape) in [
                            ("page-portrait", "Retrato", false),
                            ("page-landscape", "Paisagem", true),
                        ] {
                            let response = ui.button(label);
                            if let Some(v) = &mut self.validation {
                                v.widgets.insert(key, response.rect.center());
                            }
                            if response.clicked() {
                                action = Some(PageAction::Landscape(landscape));
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        let index = self
                            .doc
                            .pages
                            .iter()
                            .position(|p| p.id == self.doc.active_page)
                            .unwrap();
                        for (key, label, delta, enabled) in [
                            ("page-left", "Mover antes", -1, index > 0),
                            (
                                "page-right",
                                "Mover depois",
                                1,
                                index + 1 < self.doc.pages.len(),
                            ),
                        ] {
                            let response = ui.add_enabled(enabled, egui::Button::new(label));
                            if let Some(v) = &mut self.validation {
                                v.widgets.insert(key, response.rect.center());
                            }
                            if response.clicked() {
                                action = Some(PageAction::Move(delta));
                            }
                        }
                        let delete = ui.add_enabled(
                            self.doc.pages.len() > 1,
                            egui::Button::new("Excluir página…"),
                        );
                        if let Some(v) = &mut self.validation {
                            v.widgets.insert("page-delete", delete.rect.center());
                        }
                        if delete.clicked() {
                            self.pending_page_delete = Some(self.doc.active_page);
                        }
                    });
                    ui.separator();
                    let done = ui.button("Fechar");
                    if let Some(v) = &mut self.validation {
                        v.widgets.insert("page-close", done.rect.center());
                    }
                    if done.clicked() {
                        close = true;
                    }
                });
            if !open || close {
                self.page_manager = false;
                self.pending_page_delete = None;
            }
        }
        if let Some(id) = self.pending_page_delete {
            let page = self.doc.pages.iter().find(|p| p.id == id);
            if let Some(page) = page {
                let label = format!(
                    "Excluir ‘{}’ e seus {} objetos? A ação pode ser desfeita.",
                    page.name,
                    page.objects.len()
                );
                egui::Window::new("Confirmar exclusão da página")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(Align2::CENTER_CENTER, [0., 140.])
                    .show(ctx, |ui| {
                        ui.label(label);
                        ui.horizontal(|ui| {
                            let confirm = ui.button("Excluir página");
                            if let Some(v) = &mut self.validation {
                                v.widgets
                                    .insert("page-delete-confirm", confirm.rect.center());
                            }
                            if confirm.clicked() {
                                self.pending_page_delete = None;
                                action = Some(PageAction::Delete(id));
                            }
                            if ui.button("Cancelar").clicked() {
                                self.pending_page_delete = None;
                            }
                        });
                    });
            } else {
                self.pending_page_delete = None;
            }
        }
        if let Some(action) = action {
            self.page_action(action);
        }
    }
}
