use super::*;
use crate::ordering::Relative;
pub(super) struct RelativePick {
    page: u64,
    ids: Vec<u64>,
    mode: Relative,
    previous_tool: Tool,
}
impl ArtDraw {
    pub(super) fn relative_available(&self) -> bool {
        self.drag.is_none()
            && self.pen.is_empty()
            && !self.isolated_object.is_some_and(|_| {
                self.selection
                    .is_some_and(|i| self.doc.objects[i].group.is_some())
            })
            && ordering::can_target(&self.doc, &self.selected_indices())
    }
    pub(super) fn begin_relative(&mut self, mode: Relative) {
        if !self.relative_available() {
            return;
        }
        self.cancel_relative_order();
        self.cancel_gesture();
        let previous_tool = self.tool;
        self.tool = Tool::Select;
        self.pending_relative = Some(RelativePick {
            page: self.doc.active_page,
            ids: self
                .selected_indices()
                .iter()
                .map(|&i| self.doc.objects[i].id)
                .collect(),
            mode,
            previous_tool,
        });
        self.status = match mode {
            Relative::InFront => {
                "À frente do alvo: clique em outro objeto. Esc ou botão direito cancela"
            }
            Relative::Behind => {
                "Atrás do alvo: clique em outro objeto. Esc ou botão direito cancela"
            }
        }
        .into();
    }
    pub(super) fn cancel_relative_order(&mut self) {
        if let Some(pick) = self.pending_relative.take() {
            self.tool = pick.previous_tool;
            self.status = "Ordenação por alvo cancelada".into();
        }
    }
    pub(super) fn pick_relative(&mut self, world: Point) {
        let Some(pick) = &self.pending_relative else {
            return;
        };
        let (page, ids, mode) = (pick.page, pick.ids.clone(), pick.mode);
        if page != self.doc.active_page {
            self.cancel_relative_order();
            return;
        }
        let Some(indices) = ids
            .iter()
            .map(|id| self.doc.objects.iter().position(|o| o.id == *id))
            .collect::<Option<Vec<_>>>()
        else {
            self.cancel_relative_order();
            return;
        };
        let current: BTreeSet<_> = self
            .selected_indices()
            .iter()
            .map(|&i| self.doc.objects[i].id)
            .collect();
        if current != ids.iter().copied().collect() {
            self.cancel_relative_order();
            return;
        }
        // Include visible locked objects so a blocked top object never silently targets one underneath.
        let target = self
            .doc
            .paint_order()
            .into_iter()
            .rev()
            .find(|&i| self.doc.visible(i) && self.doc.objects[i].hit(world, 5. / self.zoom));
        let Some(target) = target else {
            self.status = "Clique em um objeto alvo. Esc cancela".into();
            return;
        };
        let before = self.doc.clone();
        match ordering::apply_relative(&mut self.doc, &indices, target, mode) {
            Ok(selection) => {
                self.pending_relative = None;
                self.tool = Tool::Select;
                self.set_selection(selection);
                let changed = self.doc != before;
                if changed {
                    if let Some(i) = self.selection {
                        self.doc.active_layer = self.doc.objects[i].layer;
                    }
                }
                self.remember(before);
                self.status = if changed {
                    "Seleção posicionada em relação ao alvo"
                } else {
                    "Seleção já está nessa posição"
                }
                .into();
            }
            Err(message) => self.status = format!("{message}. Esc cancela"),
        }
    }
}
