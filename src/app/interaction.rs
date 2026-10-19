use super::*;
#[cfg(test)]
use vello::kurbo::ParamCurve;
use vello::kurbo::{CubicBez, ParamCurveNearest};

impl ArtDraw {
    pub(super) fn cancel_gesture(&mut self) {
        self.cancel_relative_order();
        if let Some(before) = self.drag_before.take() {
            self.doc = before;
        }
        self.drag = None;
        self.pen.clear();
        self.draft = None;
    }
    pub(super) fn finish_pen(&mut self, closed: bool) {
        self.drag = None;
        self.draft = None;
        if self.pen.len() >= 2 {
            let before = self.doc.clone();
            let nodes = std::mem::take(&mut self.pen);
            self.doc.add_object(Object::new(
                "Curva Bézier",
                Geometry::Path { nodes, closed },
                if closed { self.fill } else { None },
                self.stroke.or(Some(INK)),
                self.stroke_width.max(0.2),
            ));
            self.set_selection(vec![self.doc.objects.len() - 1]);
            self.remember(before);
            self.status = "Caminho concluído".into();
        } else {
            self.pen.clear();
        }
    }
    fn world(&self, p: Pos2, origin: Pos2) -> Point {
        Point::new(
            f64::from(p.x - origin.x) / self.zoom,
            f64::from(p.y - origin.y) / self.zoom,
        )
    }
    fn screen(&self, p: Point, origin: Pos2) -> Pos2 {
        egui::pos2(
            origin.x + (p.x * self.zoom) as f32,
            origin.y + (p.y * self.zoom) as f32,
        )
    }
    fn origin(&self, area: ERect) -> Pos2 {
        area.center()
            - egui::vec2(
                (self.doc.width * self.zoom * 0.5) as f32,
                (self.doc.height * self.zoom * 0.5) as f32,
            )
            + self.pan
    }
    pub(super) fn canvas(&mut self, ui: &mut egui::Ui) {
        let full = ui.available_rect_before_wrap();
        ui.allocate_rect(full, Sense::hover());
        let area = ERect::from_min_max(full.min + egui::vec2(24., 24.), full.max);
        if area.width() < 4. || area.height() < 4. {
            return;
        }
        if self.fit {
            self.zoom = ((f64::from(area.width()) - 110.) / self.doc.width)
                .min((f64::from(area.height()) - 100.) / self.doc.height)
                .clamp(0.05, 8.);
            self.pan = EVec2::ZERO;
            self.fit = false;
        }
        let mut origin = self.origin(area);
        if let Some(v) = self.validation.as_mut() {
            v.view = Some((area, origin));
        }
        let pointer = ui.input(|i| i.pointer.interact_pos());
        let blocked = self.text_editor.is_some()
            || self.error.is_some()
            || self.about
            || self.recovery_dialog
            || self.page_manager
            || self.pending_page_delete.is_some();
        let hovered = ui.rect_contains_pointer(area) && !blocked;
        if hovered {
            let scroll = ui.input(|i| i.smooth_scroll_delta);
            let command = ui.input(|i| i.modifiers.command);
            if scroll != EVec2::ZERO {
                if command {
                    if let Some(p) = pointer {
                        let world = self.world(p, origin);
                        self.zoom =
                            (self.zoom * (f64::from(scroll.y) * 0.006).exp()).clamp(0.05, 16.);
                        let new_origin = area.center()
                            - egui::vec2(
                                (self.doc.width * self.zoom * 0.5) as f32,
                                (self.doc.height * self.zoom * 0.5) as f32,
                            )
                            + self.pan;
                        self.pan += p
                            - (new_origin
                                + egui::vec2(
                                    (world.x * self.zoom) as f32,
                                    (world.y * self.zoom) as f32,
                                ));
                    }
                } else {
                    self.pan += scroll;
                }
                origin = self.origin(area);
            }
            let cursor = if self.pending_relative.is_some() {
                egui::CursorIcon::Crosshair
            } else {
                match self.tool {
                    Tool::Select => egui::CursorIcon::Default,
                    Tool::Shape => egui::CursorIcon::Crosshair,
                    Tool::Hand => egui::CursorIcon::Grab,
                    Tool::Text => egui::CursorIcon::Text,
                    _ => egui::CursorIcon::Crosshair,
                }
            };
            ui.ctx().set_cursor_icon(cursor);
        }
        // The OS may deliver down, motion and up in one redraw. Anchor at the
        // actual down-event position instead of the final pointer position.
        let down = ui.input(|i| {
            i.events.iter().find_map(|event| match event {
                egui::Event::PointerButton {
                    pos,
                    button,
                    pressed: true,
                    modifiers,
                } if matches!(button, PointerButton::Primary | PointerButton::Middle) => {
                    Some((*pos, *button, *modifiers))
                }
                _ => None,
            })
        });
        let picking_target = self.pending_relative.is_some() || self.relative_click_active;
        if let Some((p, button, modifiers)) = down {
            if !blocked && area.contains(p) && ui.ctx().layer_id_at(p) == Some(ui.layer_id()) {
                let space = ui.input(|i| i.key_down(Key::Space));
                if button == PointerButton::Middle || space || self.tool == Tool::Hand {
                    self.drag = Some(Drag::Pan {
                        start: p,
                        original: self.pan,
                    });
                } else if self.pending_relative.is_some() {
                    self.relative_click_active = true;
                    self.pick_relative(self.world(p, origin));
                } else {
                    self.press(self.world(p, origin), p, origin, ui, modifiers);
                }
            }
        }
        if let Some(p) = pointer {
            let world = self.world(p, origin);
            let shift = ui.input(|i| i.modifiers.shift);
            if self.drag.is_some() {
                self.drag_to(world, p, shift);
            }

            if self.tool == Tool::Pen && !self.pen.is_empty() {
                let mut nodes = self.pen.clone();
                if !matches!(self.drag, Some(Drag::PenHandle { .. })) {
                    nodes.push(Node::corner(world));
                }
                self.draft = Some(Object::new(
                    "Caminho em edição",
                    Geometry::Path {
                        nodes,
                        closed: false,
                    },
                    None,
                    self.stroke.or(Some(INK)),
                    self.stroke_width.max(0.2),
                ));
            }
        }
        if ui.input(|i| {
            i.pointer.button_released(PointerButton::Primary)
                || i.pointer.button_released(PointerButton::Middle)
        }) {
            self.release();
        }
        if !picking_target
            && hovered
            && ui.input(|i| i.pointer.button_double_clicked(PointerButton::Primary))
        {
            if let Some(p) = pointer {
                let world = self.world(p, origin);
                if self.tool == Tool::Shape {
                    self.insert_node(world);
                }
                if self.tool == Tool::Select {
                    if let Some(i) = self.doc.hit(world, 5. / self.zoom) {
                        if matches!(self.doc.objects[i].geometry, Geometry::Text { .. }) {
                            self.edit_text(i);
                        }
                    }
                }
            }
        }
        if hovered
            && self.pending_relative.is_some()
            && ui.input(|i| i.pointer.button_clicked(PointerButton::Secondary))
        {
            self.cancel_relative_order();
        }
        if hovered
            && ui.input(|i| i.pointer.button_clicked(PointerButton::Secondary))
            && self.tool == Tool::Pen
        {
            self.finish_pen(false);
        }
        if let Err(e) = self.renderer.paint(
            ui,
            area,
            &self.doc,
            self.draft.as_ref(),
            origin,
            self.zoom,
            self.theme.canvas(),
        ) {
            self.error = Some(e);
        }
        let painter = ui.painter().with_clip_rect(area);
        if self.grid && self.zoom > 0.2 {
            let step = self.grid_step;
            let w = self.doc.width;
            let h = self.doc.height;
            let limit_x = (w / step).ceil().min(2000.) as i32;
            let limit_y = (h / step).ceil().min(2000.) as i32;
            let stroke = Stroke::new(0.5f32, Color32::from_rgba_unmultiplied(100, 130, 160, 40));
            for i in 0..=limit_x {
                let x = f64::from(i) * step;
                if x <= w {
                    painter.line_segment(
                        [
                            self.screen(Point::new(x, 0.), origin),
                            self.screen(Point::new(x, h), origin),
                        ],
                        stroke,
                    );
                }
            }
            for i in 0..=limit_y {
                let y = f64::from(i) * step;
                if y <= h {
                    painter.line_segment(
                        [
                            self.screen(Point::new(0., y), origin),
                            self.screen(Point::new(w, y), origin),
                        ],
                        stroke,
                    );
                }
            }
        }
        if let Some(Drag::Marquee { start, current, .. }) = &self.drag {
            let r = ERect::from_two_pos(self.screen(*start, origin), self.screen(*current, origin));
            painter.rect_filled(r, 0., Color32::from_rgba_unmultiplied(70, 145, 255, 25));
            painter.rect_stroke(r, 0., Stroke::new(1.0f32, ACCENT), egui::StrokeKind::Middle);
        }
        if let Some(i) = self.selection {
            if i < self.doc.objects.len() {
                self.selection_overlay(&painter, i, origin);
            }
        }
        if self.tool == Tool::Pen {
            for n in &self.pen {
                let p = self.screen(n.p, origin);
                painter.circle_filled(p, 4., ACCENT);
                painter.circle_stroke(p, 5., Stroke::new(1.0f32, Color32::WHITE));
                for h in [n.incoming, n.outgoing] {
                    if h.distance(n.p) > 0.01 {
                        let hp = self.screen(h, origin);
                        painter.line_segment([p, hp], Stroke::new(1.0f32, ACCENT));
                        painter.circle_filled(hp, 3., Color32::WHITE);
                        painter.circle_stroke(hp, 3., Stroke::new(1.0f32, ACCENT));
                    }
                }
            }
        }
        if ui.input(|i| i.pointer.button_released(PointerButton::Primary)) {
            self.relative_click_active = false;
        }
        self.rulers(ui, full, area, origin);
    }
    fn press(
        &mut self,
        world: Point,
        p: Pos2,
        origin: Pos2,
        ui: &egui::Ui,
        modifiers: egui::Modifiers,
    ) {
        if !matches!(self.tool, Tool::Select | Tool::Shape | Tool::Hand)
            && !self.active_layer_editable()
        {
            self.status = "Camada ativa oculta ou bloqueada".into();
            return;
        }
        let world = if self.snap_grid {
            Point::new(
                (world.x / self.grid_step).round() * self.grid_step,
                (world.y / self.grid_step).round() * self.grid_step,
            )
        } else {
            world
        };
        match self.tool {
            Tool::Select => {
                let additive = modifiers.shift;
                let individual = modifiers.ctrl || modifiers.command;
                if !additive && !individual {
                    if let Some(bounds) = self.selection_bounds() {
                        for (corner, q) in corners(bounds).into_iter().enumerate() {
                            if self.screen(q, origin).distance(p) < 8. {
                                self.drag_before = Some(self.doc.clone());
                                self.drag = Some(Drag::Resize {
                                    corner,
                                    from: bounds,
                                    originals: self.selected_originals(),
                                });
                                return;
                            }
                        }
                    }
                }
                let hit = self.doc.hit(world, 5. / self.zoom);
                if individual {
                    self.select_individual(hit);
                } else if !(self.isolated_object.is_some() && hit == self.selection && !additive) {
                    self.select_hit(hit, additive);
                }
                if hit.is_some() {
                    if !additive && !self.selected_indices().is_empty() {
                        self.drag_before = Some(self.doc.clone());
                        self.drag = Some(Drag::Move {
                            start: world,
                            originals: self.selected_originals(),
                        });
                    }
                    self.status =
                        format!("{} objeto(s) selecionado(s)", self.selected_indices().len());
                } else {
                    self.drag = Some(Drag::Marquee {
                        start: world,
                        current: world,
                        base: self.selected_indices(),
                    });
                }
            }
            Tool::Rectangle
            | Tool::Ellipse
            | Tool::Polygon
            | Tool::Star
            | Tool::Spiral
            | Tool::Line => {
                self.selection = None;
                self.isolated_object = None;
                self.additional.clear();
                self.drag = Some(Drag::Draw { start: world });
            }
            Tool::Text => {
                if let Some(i) = self
                    .doc
                    .hit(world, 4. / self.zoom)
                    .filter(|&i| matches!(self.doc.objects[i].geometry, Geometry::Text { .. }))
                {
                    self.select_individual(Some(i));
                    self.edit_text(i);
                } else {
                    self.text_editor = Some(TextEditor {
                        index: None,
                        position: world,
                        text: String::new(),
                        size: 36.,
                        focus: true,
                    });
                }
            }
            Tool::Pen => {
                self.selection = None;
                self.isolated_object = None;
                self.additional.clear();
                if self.pen.len() > 2 && self.screen(self.pen[0].p, origin).distance(p) < 9. {
                    self.finish_pen(true);
                    return;
                }
                if self
                    .pen
                    .last()
                    .is_some_and(|n| n.p.distance(world) * self.zoom < 2.)
                {
                    return;
                }
                self.pen.push(Node::corner(world));
                self.drag = Some(Drag::PenHandle {
                    node: self.pen.len() - 1,
                });
            }
            Tool::Shape => {
                if ui.input(|i| i.pointer.button_double_clicked(PointerButton::Primary)) {
                    self.insert_node(world);
                    return;
                }
                if let Some((index, node, part)) = self.node_hit(p, origin) {
                    self.selected_node = Some(node);
                    self.drag_before = Some(self.doc.clone());
                    self.drag = Some(Drag::Node { index, node, part });
                    return;
                }
                self.select_individual(self.doc.hit(world, 5. / self.zoom));
                self.selected_node = None;
                self.make_editable();
            }
            Tool::Freehand => {
                self.set_selection(Vec::new());
                self.drag = Some(Drag::Freehand {
                    points: vec![world],
                });
            }
            Tool::Hand => {}
        }
    }
    fn node_hit(&self, p: Pos2, origin: Pos2) -> Option<(usize, usize, u8)> {
        let i = self.selection?;
        let object = &self.doc.objects[i];
        if let Geometry::Path { nodes, .. } = &object.geometry {
            for (n, node) in nodes.iter().enumerate() {
                if self.screen(object.transform * node.p, origin).distance(p) < 8. {
                    return Some((i, n, 0));
                }
            }
            for (n, node) in nodes.iter().enumerate() {
                for (part, h) in [(1, node.incoming), (2, node.outgoing)] {
                    if h.distance(node.p) > 0.01
                        && self.screen(object.transform * h, origin).distance(p) < 7.
                    {
                        return Some((i, n, part));
                    }
                }
            }
        }
        None
    }
    fn drag_to(&mut self, world: Point, p: Pos2, shift: bool) {
        let Some(drag) = self.drag.clone() else {
            return;
        };
        let world = if self.snap_grid && !matches!(drag, Drag::Pan { .. } | Drag::Marquee { .. }) {
            Point::new(
                (world.x / self.grid_step).round() * self.grid_step,
                (world.y / self.grid_step).round() * self.grid_step,
            )
        } else {
            world
        };
        match drag {
            Drag::Pan { start, original } => {
                self.pan = original + (p - start);
            }
            Drag::Draw { start } => {
                let mut end = world;
                if shift {
                    let d = end - start;
                    let side = d.x.abs().max(d.y.abs());
                    end = start
                        + Vec2::new(
                            if d.x < 0. { -side } else { side },
                            if d.y < 0. { -side } else { side },
                        );
                }
                let bounds = Rect::from_points(start, end);
                let geometry = match self.tool {
                    Tool::Rectangle => Geometry::Rectangle(bounds),
                    Tool::Ellipse => Geometry::Ellipse(bounds),
                    Tool::Polygon => editing::polygon(bounds, self.shape_sides, false),
                    Tool::Star => editing::polygon(bounds, self.shape_sides, true),
                    Tool::Spiral => editing::spiral(bounds, self.shape_sides),
                    _ => {
                        if shift {
                            let d = world - start;
                            let angle = (d.y.atan2(d.x) / std::f64::consts::FRAC_PI_4).round()
                                * std::f64::consts::FRAC_PI_4;
                            end = start + Vec2::new(angle.cos(), angle.sin()) * d.hypot();
                        }
                        Geometry::Path {
                            nodes: vec![Node::corner(start), Node::corner(end)],
                            closed: false,
                        }
                    }
                };
                self.draft = Some(Object::new(
                    self.tool.label(),
                    geometry,
                    if matches!(self.tool, Tool::Line | Tool::Spiral) {
                        None
                    } else {
                        self.fill
                    },
                    self.stroke
                        .or(if matches!(self.tool, Tool::Line | Tool::Spiral) {
                            Some(INK)
                        } else {
                            None
                        }),
                    self.stroke_width,
                ));
                self.status = format!(
                    "{} × {} px",
                    bounds.width().round(),
                    bounds.height().round()
                );
            }
            Drag::Marquee { start, base, .. } => {
                self.drag = Some(Drag::Marquee {
                    start,
                    current: world,
                    base,
                });
            }
            Drag::Freehand { mut points } => {
                if points
                    .last()
                    .is_none_or(|p| p.distance(world) * self.zoom > 2.)
                {
                    points.push(world);
                }
                self.draft = Some(Object::new(
                    "Mão livre",
                    Geometry::Path {
                        nodes: points.iter().copied().map(Node::corner).collect(),
                        closed: false,
                    },
                    None,
                    self.stroke.or(Some(INK)),
                    self.stroke_width.max(0.2),
                ));
                self.drag = Some(Drag::Freehand { points });
            }
            Drag::Move { start, originals } => {
                let mut delta = world - start;
                if shift {
                    if delta.x.abs() > delta.y.abs() {
                        delta.y = 0.;
                    } else {
                        delta.x = 0.;
                    }
                }
                for (index, original) in originals {
                    self.doc.objects[index] = original;
                    self.doc.objects[index].translate(delta);
                }
            }
            Drag::Resize {
                corner,
                from,
                originals,
            } => {
                let opposite = corners(from)[(corner + 2) % 4];
                let mut end = world;
                if shift {
                    let d = end - opposite;
                    let ratio = from.width().max(0.1) / from.height().max(0.1);
                    let width = d.x.abs().max(d.y.abs() * ratio).max(0.1);
                    end = opposite
                        + Vec2::new(
                            if d.x < 0. { -width } else { width },
                            if d.y < 0. {
                                -width / ratio
                            } else {
                                width / ratio
                            },
                        );
                }
                let mut to = Rect::from_points(opposite, end);
                if to.width() < 0.1 {
                    to.x1 = to.x0 + 0.1;
                }
                if to.height() < 0.1 {
                    to.y1 = to.y0 + 0.1;
                }
                for (index, original) in originals {
                    self.doc.objects[index].resize_from(&original, from, to);
                }
            }
            Drag::Node { index, node, part } => {
                let object = &mut self.doc.objects[index];
                if object.transform.determinant().abs() < 1e-12 {
                    return;
                }
                let local = object.transform.inverse() * world;
                if let Geometry::Path { nodes, .. } = &mut object.geometry {
                    let n = &mut nodes[node];
                    match part {
                        0 => n.translate(local - n.p),
                        1 => {
                            n.incoming = local;
                            if shift {
                                n.outgoing = n.p + (n.p - local);
                            }
                        }
                        _ => {
                            n.outgoing = local;
                            if shift {
                                n.incoming = n.p + (n.p - local);
                            }
                        }
                    }
                }
            }
            Drag::PenHandle { node } => {
                if let Some(n) = self.pen.get_mut(node) {
                    let mut d = world - n.p;
                    if d.hypot() * self.zoom < 3. {
                        d = Vec2::ZERO;
                    }
                    if shift {
                        let angle = (d.y.atan2(d.x) / std::f64::consts::FRAC_PI_4).round()
                            * std::f64::consts::FRAC_PI_4;
                        d = Vec2::new(angle.cos(), angle.sin()) * d.hypot();
                    }
                    n.outgoing = n.p + d;
                    n.incoming = n.p - d;
                }
            }
        }
    }
    fn release(&mut self) {
        if let Some(Drag::Marquee {
            start,
            current,
            base,
        }) = self.drag.clone()
        {
            let mut ids = base;
            ids.extend(editing::marquee(
                &self.doc,
                Rect::from_points(start, current),
                current.x < start.x,
            ));
            self.set_selection(ids);
        }
        if matches!(self.drag, Some(Drag::Draw { .. } | Drag::Freehand { .. })) {
            if let Some(o) = self.draft.take() {
                let b = o.bounds();
                if b.width().hypot(b.height()) > 0.5 {
                    let before = self.doc.clone();
                    self.doc.add_object(o);
                    self.set_selection(vec![self.doc.objects.len() - 1]);
                    self.remember(before);
                    self.status = "Objeto criado".into();
                }
            }
        }
        if let Some(before) = self.drag_before.take() {
            self.remember(before);
        }
        self.drag = None;
        if self.tool != Tool::Pen {
            self.draft = None;
        }
    }
    fn insert_node(&mut self, world: Point) {
        let Some(i) = self.selection else {
            return;
        };
        let object = &self.doc.objects[i];
        let Geometry::Path { nodes, closed } = &object.geometry else {
            return;
        };
        if nodes.len() < 2 {
            return;
        }
        let mut best: Option<(usize, f64, f64)> = None;
        for seg in 0..(nodes.len() - 1 + usize::from(*closed)) {
            let a = &nodes[seg];
            let b = &nodes[(seg + 1) % nodes.len()];
            let curve = CubicBez::new(
                object.transform * a.p,
                object.transform * a.outgoing,
                object.transform * b.incoming,
                object.transform * b.p,
            );
            let near = curve.nearest(world, 0.01);
            if best.is_none_or(|(_, _, distance)| near.distance_sq < distance) {
                best = Some((seg, near.t, near.distance_sq));
            }
        }
        let Some((segment, t, dist)) = best else {
            return;
        };
        if dist.sqrt() * self.zoom > 10. || !(0.01..0.99).contains(&t) {
            return;
        }
        let before = self.doc.clone();
        if let Geometry::Path { nodes, .. } = &mut self.doc.objects[i].geometry {
            let next = (segment + 1) % nodes.len();
            let a = &nodes[segment];
            let b = &nodes[next];
            let q0 = a.p.lerp(a.outgoing, t);
            let q1 = a.outgoing.lerp(b.incoming, t);
            let q2 = b.incoming.lerp(b.p, t);
            let r0 = q0.lerp(q1, t);
            let r1 = q1.lerp(q2, t);
            let position = r0.lerp(r1, t);
            nodes[segment].outgoing = q0;
            nodes[next].incoming = q2;
            nodes.insert(
                segment + 1,
                Node {
                    p: position,
                    incoming: r0,
                    outgoing: r1,
                },
            );
            self.selected_node = Some(segment + 1);
        }
        self.remember(before);
        self.status = "Nó inserido preservando a curva".into();
    }
    fn selection_overlay(&self, painter: &egui::Painter, index: usize, origin: Pos2) {
        let object = &self.doc.objects[index];
        if self.tool == Tool::Shape {
            if let Geometry::Path { nodes, .. } = &object.geometry {
                for (n, node) in nodes.iter().enumerate() {
                    let p = self.screen(object.transform * node.p, origin);
                    for h in [node.incoming, node.outgoing] {
                        if h.distance(node.p) > 0.01 {
                            let hp = self.screen(object.transform * h, origin);
                            painter.line_segment([p, hp], Stroke::new(1.0f32, ACCENT));
                            painter.circle_filled(hp, 3.5, Color32::WHITE);
                            painter.circle_stroke(hp, 3.5, Stroke::new(1.3f32, ACCENT));
                        }
                    }
                    let r = ERect::from_center_size(p, egui::vec2(8., 8.));
                    painter.rect_filled(
                        r,
                        1.,
                        if self.selected_node == Some(n) {
                            ACCENT
                        } else {
                            Color32::WHITE
                        },
                    );
                    painter.rect_stroke(
                        r,
                        1.,
                        Stroke::new(1.4f32, ACCENT),
                        egui::StrokeKind::Middle,
                    );
                }
                return;
            }
        }
        let b = self.selection_bounds().unwrap_or_else(|| object.bounds());
        let r = ERect::from_min_max(
            self.screen(Point::new(b.x0, b.y0), origin),
            self.screen(Point::new(b.x1, b.y1), origin),
        );
        painter.rect_stroke(
            r,
            0.,
            Stroke::new(1.0f32, ACCENT),
            egui::StrokeKind::Outside,
        );
        for q in corners(b) {
            let h = ERect::from_center_size(self.screen(q, origin), egui::vec2(7., 7.));
            painter.rect_filled(h, 1., Color32::WHITE);
            painter.rect_stroke(h, 1., Stroke::new(1.4f32, ACCENT), egui::StrokeKind::Middle);
        }
    }
    fn rulers(&self, ui: &egui::Ui, full: ERect, area: ERect, origin: Pos2) {
        let p = ui.painter();
        let bg = self.theme.chrome();
        let color = Color32::from_gray(160);
        let top = ERect::from_min_max(full.min, egui::pos2(full.max.x, area.min.y));
        let left = ERect::from_min_max(full.min, egui::pos2(area.min.x, full.max.y));
        p.rect_filled(top, 0., bg);
        p.rect_filled(left, 0., bg);
        let unit = if self.zoom < 0.15 {
            500.
        } else if self.zoom < 0.35 {
            200.
        } else if self.zoom < 0.8 {
            100.
        } else if self.zoom < 2. {
            50.
        } else if self.zoom < 5. {
            20.
        } else {
            10.
        };
        let top_p = p.with_clip_rect(top);
        let left_p = p.with_clip_rect(left);
        let start_x = ((f64::from(area.min.x - origin.x) / self.zoom) / unit).floor() as i64;
        let end_x = ((f64::from(area.max.x - origin.x) / self.zoom) / unit).ceil() as i64;
        for i in start_x..=end_x {
            let x = origin.x + (i as f64 * unit * self.zoom) as f32;
            top_p.line_segment(
                [egui::pos2(x, area.min.y - 7.), egui::pos2(x, area.min.y)],
                Stroke::new(1.0f32, color),
            );
            top_p.text(
                egui::pos2(x + 3., top.min.y + 3.),
                Align2::LEFT_TOP,
                format!("{}", (i as f64 * unit) as i64),
                FontId::proportional(10.),
                color,
            );
        }
        let start_y = ((f64::from(area.min.y - origin.y) / self.zoom) / unit).floor() as i64;
        let end_y = ((f64::from(area.max.y - origin.y) / self.zoom) / unit).ceil() as i64;
        for i in start_y..=end_y {
            let y = origin.y + (i as f64 * unit * self.zoom) as f32;
            left_p.line_segment(
                [egui::pos2(area.min.x - 6., y), egui::pos2(area.min.x, y)],
                Stroke::new(1.0f32, color),
            );
            left_p.text(
                egui::pos2(full.min.x + 2., y + 3.),
                Align2::LEFT_TOP,
                format!("{}", (i as f64 * unit) as i64),
                FontId::proportional(8.),
                color,
            );
        }
        p.rect_filled(
            ERect::from_min_max(full.min, area.min),
            0.,
            Color32::from_rgb(46, 50, 59),
        );
    }
}
fn corners(r: Rect) -> [Point; 4] {
    [
        Point::new(r.x0, r.y0),
        Point::new(r.x1, r.y0),
        Point::new(r.x1, r.y1),
        Point::new(r.x0, r.y1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn de_casteljau_split_is_exact() {
        let c = CubicBez::new((0., 0.), (20., 100.), (80., -40.), (100., 20.));
        let t = 0.37;
        let q0 = c.p0.lerp(c.p1, t);
        let q1 = c.p1.lerp(c.p2, t);
        let q2 = c.p2.lerp(c.p3, t);
        let r0 = q0.lerp(q1, t);
        let r1 = q1.lerp(q2, t);
        let s = r0.lerp(r1, t);
        let a = CubicBez::new(c.p0, q0, r0, s);
        let b = CubicBez::new(s, r1, q2, c.p3);
        for i in 0..=10 {
            let u = i as f64 / 10.;
            assert!(a.eval(u).distance(c.eval(u * t)) < 1e-9);
            assert!(b.eval(u).distance(c.eval(t + u * (1. - t))) < 1e-9);
        }
    }
}
