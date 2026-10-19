use super::*;
use crate::document::Layer;
use vello::kurbo::Affine;
impl ArtDraw {
    pub(super) fn selected_indices(&self) -> Vec<usize> {
        let mut indices = self.additional.clone();
        if let Some(i) = self.selection {
            indices.insert(i);
        }
        if self.isolated_object.is_some_and(|id| {
            self.selection
                .is_some_and(|i| self.doc.objects.get(i).is_some_and(|o| o.id == id))
        }) {
            indices
                .into_iter()
                .filter(|&i| self.doc.editable(i))
                .collect()
        } else {
            editing::expand_groups(&self.doc, indices)
        }
    }
    pub(super) fn set_selection(&mut self, indices: Vec<usize>) {
        self.isolated_object = None;
        let previous = self.selection;
        let old = self.additional.clone();
        self.additional = indices
            .into_iter()
            .filter(|&i| self.doc.editable(i))
            .collect();
        self.selection = self.additional.iter().next_back().copied();
        if let Some(i) = self.selection {
            self.additional.remove(&i);
        }
        if previous != self.selection || old != self.additional {
            self.cancel_relative_order();
            self.selected_node = None;
        }
    }
    pub(super) fn select_individual(&mut self, index: Option<usize>) {
        self.set_selection(index.into_iter().collect());
        self.isolated_object = self.selection.map(|i| self.doc.objects[i].id);
        self.status = if self.selection.is_some() {
            "Folha selecionada; irmãos preservados"
        } else {
            "Nenhum objeto editável"
        }
        .into();
    }
    fn sanitize_selection(&mut self) {
        let isolated = self.isolated_object;
        let indices = self.selected_indices();
        self.set_selection(indices);
        if isolated.is_some_and(|id| self.selection.is_some_and(|i| self.doc.objects[i].id == id)) {
            self.isolated_object = isolated;
        }
    }
    pub(super) fn selection_bounds(&self) -> Option<Rect> {
        editing::bounds(&self.doc, &self.selected_indices())
    }
    pub(super) fn selected_originals(&self) -> Vec<(usize, Object)> {
        self.selected_indices()
            .into_iter()
            .map(|i| (i, self.doc.objects[i].clone()))
            .collect()
    }
    pub(super) fn select_hit(&mut self, index: Option<usize>, additive: bool) {
        self.isolated_object = None;
        let mut current: BTreeSet<_> = self.selected_indices().into_iter().collect();
        if let Some(i) = index {
            let unit = editing::expand_groups(&self.doc, [i]);
            if additive {
                if unit.iter().all(|j| current.contains(j)) {
                    for j in unit {
                        current.remove(&j);
                    }
                } else {
                    current.extend(unit)
                }
            } else if !current.contains(&i) {
                current = unit.into_iter().collect();
            }
        } else if !additive {
            current.clear();
        }
        self.set_selection(current.into_iter().collect());
    }
    pub(super) fn resize_selection(&mut self, from: Rect, to: Rect) {
        for (i, o) in self.selected_originals() {
            self.doc.objects[i].resize_from(&o, from, to);
        }
    }
    pub(super) fn copy_selection(&mut self) {
        self.clipboard = self
            .selected_indices()
            .into_iter()
            .map(|i| self.doc.objects[i].clone())
            .collect();
        self.clipboard_parents = self.doc.group_parents.clone();
        if self.isolated_object.is_some() {
            for object in &mut self.clipboard {
                object.group = None;
            }
            self.clipboard_parents.clear();
        }
        self.status = format!("{} objetos copiados no ArtDraw", self.clipboard.len());
    }
    pub(super) fn paste_selection(&mut self) {
        if !self.active_layer_editable() {
            self.status = "Ative uma camada visível e desbloqueada".into();
            return;
        }
        let before = self.doc.clone();
        let objects = self.clipboard.clone();
        let ids = editing::duplicate(
            &mut self.doc,
            &objects,
            &self.clipboard_parents,
            Vec2::new(16., 16.),
        );
        self.set_selection(ids);
        self.remember(before);
    }
    pub(super) fn active_layer_editable(&self) -> bool {
        self.doc
            .layers
            .iter()
            .any(|l| l.id == self.doc.active_layer && l.visible && !l.locked)
    }
    pub(super) fn group_selection(&mut self) {
        let indices = self.selected_indices();
        if indices.len() < 2 {
            return;
        }
        let before = self.doc.clone();
        let ids = editing::group(&mut self.doc, &indices);
        self.set_selection(ids);
        let changed = self.doc != before;
        self.remember(before);
        self.status = if changed {
            "Grupo criado; subgrupos preservados"
        } else {
            "Selecione duas unidades; máximo64 níveis"
        }
        .into();
    }
    pub(super) fn ungroup_selection(&mut self) {
        self.ungroup_levels(false);
    }
    pub(super) fn ungroup_levels(&mut self, all: bool) {
        if self.isolated_object.is_some() {
            self.status = "Selecione o conjunto para desagrupar".into();
            return;
        }
        let indices = self.selected_indices();
        let before = self.doc.clone();
        editing::ungroup(&mut self.doc, &indices, all);
        self.set_selection(indices);
        self.remember(before);
        self.status = if all {
            "Todos os níveis separados"
        } else {
            "Nível externo separado; subgrupos preservados"
        }
        .into();
    }
    pub(super) fn order_selection(&mut self, front: bool) {
        self.order_command(if front {
            Order::FrontPage
        } else {
            Order::BackPage
        });
    }
    fn order_availability(&self) -> ordering::Availability {
        if self.isolated_object.is_some() && self.selection.is_some_and(|i| self.doc.objects[i].group.is_some()) {
            return ordering::Availability::default();
        }
        ordering::availability(&self.doc, &self.selected_indices())
    }
    pub(super) fn order_command(&mut self, mode: Order) {
        if self.isolated_object.is_some_and(|_| {
            self.selection
                .is_some_and(|i| self.doc.objects[i].group.is_some())
        }) {
            self.status =
                "Ordenação interna do grupo ainda não disponível; selecione o conjunto".into();
            return;
        }
        let before = self.doc.clone();
        let indices = self.selected_indices();
        match ordering::apply(&mut self.doc, &indices, mode) {
            Ok(ids) => {
                let changed = self.doc != before;
                self.set_selection(ids);
                if changed {
                    if let Some(i) = self.selection {
                        self.doc.active_layer = self.doc.objects[i].layer;
                    }
                }
                self.remember(before);
                self.status = if changed {
                    "Ordem alterada; grupos e atributos preservados"
                } else {
                    if matches!(mode, Order::Reverse) {
                        "Selecione duas ou mais unidades da mesma camada"
                    } else {
                        "Seleção já está no limite permitido"
                    }
                }
                .into();
            }
            Err(error) => self.error = Some(error),
        }
    }
    pub(super) fn rotate_selection(&mut self, degrees: f64) {
        let Some(b) = self.selection_bounds() else {
            return;
        };
        let c = b.center().to_vec2();
        let before = self.doc.clone();
        let ids = self.selected_indices();
        editing::transform(
            &mut self.doc,
            &ids,
            Affine::translate(c) * Affine::rotate(degrees.to_radians()) * Affine::translate(-c),
        );
        self.remember(before);
    }
    pub(super) fn mirror_selection(&mut self, horizontal: bool) {
        let Some(b) = self.selection_bounds() else {
            return;
        };
        let c = b.center().to_vec2();
        let before = self.doc.clone();
        let ids = self.selected_indices();
        editing::transform(
            &mut self.doc,
            &ids,
            Affine::translate(c)
                * Affine::scale_non_uniform(
                    if horizontal { -1. } else { 1. },
                    if horizontal { 1. } else { -1. },
                )
                * Affine::translate(-c),
        );
        self.remember(before);
    }
    pub(super) fn skew_selection(&mut self, degrees: f64) {
        let Some(b) = self.selection_bounds() else {
            return;
        };
        let c = b.center().to_vec2();
        let before = self.doc.clone();
        let ids = self.selected_indices();
        editing::transform(
            &mut self.doc,
            &ids,
            Affine::translate(c)
                * Affine::new([
                    1.,
                    0.,
                    degrees.clamp(-80., 80.).to_radians().tan(),
                    1.,
                    0.,
                    0.,
                ])
                * Affine::translate(-c),
        );
        self.remember(before);
    }
    pub(super) fn align_selection(&mut self, mode: editing::Align) {
        let before = self.doc.clone();
        let ids = self.selected_indices();
        editing::align(&mut self.doc, &ids, mode, self.align_page);
        self.remember(before);
    }
    pub(super) fn distribute_selection(&mut self, horizontal: bool) {
        let before = self.doc.clone();
        let ids = self.selected_indices();
        editing::distribute(&mut self.doc, &ids, horizontal);
        self.remember(before);
    }
    pub(super) fn object_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Objeto", |ui| {
            let has = !self.selected_indices().is_empty();
            if ui.button("Selecionar tudo    Ctrl+A").clicked() {
                self.set_selection(editing::expand_groups(&self.doc, 0..self.doc.objects.len()));
                ui.close_menu();
            }
            ui.add_enabled_ui(has, |ui| {
                if ui.button("Agrupar     Ctrl+G").clicked() {
                    self.group_selection();
                    ui.close_menu();
                }
                if ui.button("Desagrupar     Ctrl+U").clicked() {
                    self.ungroup_selection();
                    ui.close_menu();
                }
                if ui.button("Desagrupar todos os níveis").clicked() {
                    self.ungroup_levels(true);
                    ui.close_menu();
                }
                if ui.button("Copiar     Ctrl+C").clicked() {
                    self.copy_selection();
                    ui.close_menu();
                }
                if ui.button("Recortar     Ctrl+X").clicked() {
                    self.copy_selection();
                    self.delete();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Girar 90°").clicked() {
                    self.rotate_selection(90.);
                    ui.close_menu();
                }
                if ui.button("Espelhar horizontal").clicked() {
                    self.mirror_selection(true);
                    ui.close_menu();
                }
                if ui.button("Espelhar vertical").clicked() {
                    self.mirror_selection(false);
                    ui.close_menu();
                }
                ui.menu_button("Ordem", |ui| {
                    let availability = self.order_availability();
                    for (label, mode) in [
                        ("Frente da página   Ctrl+Home", Order::FrontPage),
                        ("Trás da página   Ctrl+End", Order::BackPage),
                        ("Frente da camada   Shift+PgUp", Order::FrontLayer),
                        ("Trás da camada   Shift+PgDown", Order::BackLayer),
                        ("Avançar um nível   Ctrl+PgUp", Order::ForwardOne),
                        ("Recuar um nível   Ctrl+PgDown", Order::BackOne),
                    ] {
                        if ui.add_enabled(availability.allows(mode), egui::Button::new(label)).clicked() {
                            self.order_command(mode);
                            ui.close_menu();
                        }
                    }
                    ui.separator();
                    for (label, mode) in [("À frente de um alvo…", ordering::Relative::InFront), ("Atrás de um alvo…", ordering::Relative::Behind)] {
                        if ui.add_enabled(self.relative_available(), egui::Button::new(label)).clicked() { self.begin_relative(mode); ui.close_menu(); }
                    }
                    ui.separator();
                    let available=availability.allows(Order::Reverse);
                    if ui.add_enabled(available,egui::Button::new("Inverter ordem"))
                        .on_disabled_hover_text("Selecione duas ou mais unidades da mesma camada; um grupo conta como uma unidade")
                        .clicked(){self.order_command(Order::Reverse);ui.close_menu();}
                });
            });
            if ui
                .add_enabled(
                    !self.clipboard.is_empty(),
                    egui::Button::new("Colar     Ctrl+V"),
                )
                .clicked()
            {
                self.paste_selection();
                ui.close_menu();
            }
        });
    }
    /// Display each immediate child once, while preserving reverse paint order.
    fn object_tree(
        &mut self,
        ui: &mut egui::Ui,
        parent: Option<u64>,
        layer: u64,
        tree: &ObjectTree,
    ) -> Option<(usize, bool)> {
        let mut pick = None;
        for row in tree
            .children
            .get(&(layer, parent))
            .cloned()
            .unwrap_or_default()
        {
            let i = row.index;
            if let Some(group) = row.group {
                let members = tree.members[&group].clone();
                let mut visible = members.iter().all(|&j| self.doc.objects[j].visible);
                let mut locked = members.iter().all(|&j| self.doc.objects[j].locked);
                ui.push_id(("group", group), |ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .checkbox(&mut visible, "V")
                            .on_hover_text("Mostrar/ocultar todos os descendentes")
                            .changed()
                        {
                            for &j in &members {
                                self.doc.objects[j].visible = visible;
                            }
                        }
                        if ui
                            .checkbox(&mut locked, "B")
                            .on_hover_text("Bloquear/desbloquear todos os descendentes")
                            .changed()
                        {
                            for &j in &members {
                                self.doc.objects[j].locked = locked;
                            }
                        }
                        if ui
                            .small_button("Selecionar")
                            .on_hover_text("Selecionar o conjunto externo")
                            .clicked()
                        {
                            pick = Some((i, false));
                        }
                    });
                    egui::CollapsingHeader::new(format!(
                        "Grupo {group} · {} objetos",
                        members.len()
                    ))
                    .id_salt(group)
                    .default_open(true)
                    .show(ui, |ui| {
                        if let Some(i) = self.object_tree(ui, Some(group), layer, tree) {
                            pick = Some(i);
                        }
                    });
                });
            } else {
                let selected = self.selection == Some(i) || self.additional.contains(&i);
                let o = &mut self.doc.objects[i];
                ui.push_id(("object", o.id), |ui| {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut o.visible, "V");
                        ui.checkbox(&mut o.locked, "B");
                        if ui
                            .selectable_label(selected, &o.name)
                            .on_hover_text(
                                "Editar somente esta folha; Ctrl+clique no canvas também isola",
                            )
                            .clicked()
                        {
                            pick = Some((i, true));
                        }
                    });
                });
            }
        }
        pick
    }
    pub(super) fn inspector(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("inspector").default_width(264.).min_width(232.).max_width(380.).resizable(true).show(ctx,|ui|{
 if self.text_editor.is_some(){ui.disable();}ui.add_space(8.);ui.heading("Objetos");
 egui::ScrollArea::vertical().id_salt("professional").show(ui,|ui|{
  let layer_header=ui.collapsing("Camadas",|ui|{let before=self.doc.clone();let mut active=self.doc.active_layer;for l in &mut self.doc.layers{ui.push_id(l.id,|ui|{ui.horizontal(|ui|{let visible=ui.checkbox(&mut l.visible,"V").on_hover_text("Visível");let locked=ui.checkbox(&mut l.locked,"B").on_hover_text("Bloqueada");if l.id==active{if let Some(v)=self.validation.as_mut(){v.widgets.insert("layer-visible",visible.rect.center());v.widgets.insert("layer-locked",locked.rect.center());}}if ui.selectable_label(active==l.id,&l.name).clicked(){active=l.id;}});});}self.doc.active_layer=active;
   ui.horizontal(|ui|{let response=ui.small_button("+ Camada");if let Some(v)=self.validation.as_mut(){v.widgets.insert("new-layer",response.rect.center());}if response.clicked(){let id=self.doc.layers.iter().map(|l|l.id).max().unwrap_or(0)+1;self.doc.layers.push(Layer{id,name:format!("Camada {id}"),visible:true,locked:false});self.doc.active_layer=id;}});
   if let Some(index)=self.doc.layers.iter().position(|l|l.id==self.doc.active_layer){ui.text_edit_singleline(&mut self.doc.layers[index].name);ui.horizontal(|ui|{if ui.add_enabled(index+1<self.doc.layers.len(),egui::Button::new("Subir")).clicked(){self.doc.layers.swap(index,index+1);}if ui.add_enabled(index>0,egui::Button::new("Descer")).clicked(){self.doc.layers.swap(index,index-1);}let empty=!self.doc.objects.iter().any(|o|o.layer==self.doc.active_layer);if ui.add_enabled(empty&&self.doc.layers.len()>1,egui::Button::new("Excluir vazia")).clicked(){self.doc.layers.remove(index);self.doc.active_layer=self.doc.layers[0].id;}});}
   self.remember(before);self.sanitize_selection();
  });
  if let Some(v)=self.validation.as_mut(){v.widgets.insert("layers-header",layer_header.header_response.rect.center());}
  let layer_name=self.doc.layers.iter().find(|l|l.id==self.doc.active_layer).map(|l|l.name.as_str()).unwrap_or("");ui.label(egui::RichText::new(layer_name).strong());
  let before=self.doc.clone();let mut pick=None;
  egui::ScrollArea::vertical().id_salt("objects_list").max_height(180.).show(ui,|ui|{
   let tree=ObjectTree::new(&self.doc);pick=self.object_tree(ui,None,self.doc.active_layer,&tree);
  });self.remember(before);
  if let Some((i,individual))=pick{if individual{self.select_individual(Some(i));}else{self.select_hit(Some(i),ui.input(|i|i.modifiers.shift));}if self.tool==Tool::Shape{self.make_editable();}}
  self.sanitize_selection();
  let count=self.selected_indices().len();ui.label(format!("{count} selecionado(s) · {} objetos",self.doc.objects.len()));if self.isolated_object.is_some(){ui.label(egui::RichText::new("Editando folha · irmãos preservados").small());if ui.button("Selecionar conjunto").clicked(){let i=self.selection;self.select_hit(i,false);}}ui.separator();
  let(fill,stroke)=self.selection.map(|i|(self.doc.objects[i].fill,self.doc.objects[i].stroke)).unwrap_or((self.fill,self.stroke));
  for(label,c,is_stroke)in[("Preenchimento",fill,false),("Contorno",stroke,true)]{ui.horizontal(|ui|{let mut value=c.unwrap_or([0,0,0,255]);if ui.color_edit_button_srgba_unmultiplied(&mut value).changed(){self.apply_color(Some(value),is_stroke);}ui.label(label);let clear=ui.small_button("×");if let Some(v)=self.validation.as_mut(){v.widgets.insert(if is_stroke{"clear-stroke"}else{"clear-fill"},clear.rect.center());}if clear.clicked(){self.apply_color(None,is_stroke);}});}
  if let Some(i)=self.selection{let before=self.doc.clone();if count==1{ui.text_edit_singleline(&mut self.doc.objects[i].name);if matches!(self.doc.objects[i].geometry,Geometry::Text{..})&&ui.button("Editar texto…").clicked(){self.edit_text(i);}}self.remember(before);}
  ui.add_enabled_ui(count>0,|ui|{
   ui.horizontal(|ui|{if ui.button("Agrupar").clicked(){self.group_selection();}if ui.button("Separar").on_hover_text("Desagrupar um nível, preservando subgrupos").clicked(){self.ungroup_selection();}});
   let all=ui.button("Separar todos os níveis");if let Some(v)=self.validation.as_mut(){v.widgets.insert("ungroup-all",all.rect.center());}if all.clicked(){self.ungroup_levels(true);}
   ui.horizontal(|ui|{if ui.button("Duplicar").clicked(){self.duplicate();}if ui.button("Excluir").clicked(){self.delete();}});
   let availability = self.order_availability();
   ui.horizontal(|ui| {
       for (label, tip, mode, key) in [
           ("Frente", "Frente da página · Ctrl+Home", Order::FrontPage, "order-front"),
           ("Trás", "Trás da página · Ctrl+End", Order::BackPage, "order-back"),
           ("Subir 1", "Ctrl+PgUp · cruza camada no limite", Order::ForwardOne, "order-up"),
           ("Descer 1", "Ctrl+PgDown · cruza camada no limite", Order::BackOne, "order-down"),
       ] {
           let response = ui.add_enabled(availability.allows(mode), egui::Button::new(label).small()).on_hover_text(tip);
           if let Some(v) = self.validation.as_mut() { v.widgets.insert(key, response.rect.center()); v.order_enabled[mode as usize] = response.enabled(); }
           if response.clicked() { match mode { Order::FrontPage => self.order_selection(true), Order::BackPage => self.order_selection(false), _ => self.order_command(mode) } }
       }
   });
   let available=availability.allows(Order::Reverse);
   let reverse=ui.add_enabled(available,egui::Button::new("Inverter ordem"))
      .on_hover_text("Inverte apenas as unidades selecionadas da mesma camada; grupos preservados")
      .on_disabled_hover_text("Selecione duas ou mais unidades da mesma camada; grupo conta como uma unidade");
   if let Some(v)=self.validation.as_mut(){v.widgets.insert("order-reverse",reverse.rect.center());}
   if reverse.clicked(){self.order_command(Order::Reverse);}
   ui.horizontal(|ui| {
       for (label, mode, key) in [("À frente do alvo…", ordering::Relative::InFront, "relative-front"), ("Atrás do alvo…", ordering::Relative::Behind, "relative-behind")] {
           let r = ui.add_enabled(self.relative_available(), egui::Button::new(label).small());
           if let Some(v) = self.validation.as_mut() { v.widgets.insert(key, r.rect.center()); }
           if r.clicked() { self.begin_relative(mode); }
       }
   });
   if self.pending_relative.is_some() { ui.label(egui::RichText::new("Clique no alvo no desenho · Esc cancela").color(ACCENT)); }

   let transform_header=ui.collapsing("Transformações",|ui|{ui.horizontal(|ui|{ui.add(egui::DragValue::new(&mut self.rotation_delta).suffix("°").range(-360.0..=360.0));let rotate=ui.button("Girar");if let Some(v)=self.validation.as_mut(){v.widgets.insert("rotate",rotate.rect.center());}if rotate.clicked(){self.rotate_selection(self.rotation_delta);}});ui.horizontal(|ui|{ui.add(egui::DragValue::new(&mut self.skew_delta).suffix("°").range(-80.0..=80.0));if ui.button("Inclinar").clicked(){self.skew_selection(self.skew_delta);}});ui.horizontal(|ui|{let mirror=ui.button("Espelho H");if let Some(v)=self.validation.as_mut(){v.widgets.insert("mirror-h",mirror.rect.center());}if mirror.clicked(){self.mirror_selection(true);}if ui.button("Espelho V").clicked(){self.mirror_selection(false);}});});
   if let Some(v)=self.validation.as_mut(){v.widgets.insert("transforms-header",transform_header.header_response.rect.center());}
   let align_header=ui.collapsing("Alinhar e distribuir",|ui|{let target=ui.checkbox(&mut self.align_page,"Usar página como referência");if let Some(v)=self.validation.as_mut(){v.widgets.insert("align-page",target.rect.center());}for row in [[("Esq.",editing::Align::Left),("Centro H",editing::Align::CenterX),("Dir.",editing::Align::Right)],[("Topo",editing::Align::Top),("Centro V",editing::Align::CenterY),("Base",editing::Align::Bottom)]]{ui.horizontal(|ui|{for(label,mode)in row{let response=ui.small_button(label);if label=="Centro H"{if let Some(v)=self.validation.as_mut(){v.widgets.insert("align-center",response.rect.center());}}if response.clicked(){self.align_selection(mode);}}});}ui.horizontal(|ui|{if ui.small_button("Distribuir H").clicked(){self.distribute_selection(true);}if ui.small_button("Distribuir V").clicked(){self.distribute_selection(false);}});ui.label(egui::RichText::new("Alinhar: caixa coletiva ou página. Distribuir: centros de objetos/grupos.").small());});
   if let Some(v)=self.validation.as_mut(){v.widgets.insert("align-header",align_header.header_response.rect.center());}
  });ui.separator();ui.label(egui::RichText::new(self.tool.hint()).small());ui.label(egui::RichText::new("Shift+clique: adicionar/remover. Arrasto no vazio: seleção por área. Paleta: esquerdo preenche, direito contorna.").small());
 });});
    }
}

/// Build adjacency once per panel frame, instead of scanning all objects for every group.
#[derive(Clone, Copy)]
struct TreeRow {
    index: usize,
    group: Option<u64>,
}
#[derive(Default)]
struct ObjectTree {
    children: std::collections::BTreeMap<(u64, Option<u64>), Vec<TreeRow>>,
    members: std::collections::BTreeMap<u64, Vec<usize>>,
}
impl ObjectTree {
    fn new(doc: &Document) -> Self {
        let mut tree = Self::default();
        let mut seen = BTreeSet::new();
        for i in doc.paint_order().into_iter().rev() {
            let object = &doc.objects[i];
            let layer = object.layer;
            let mut parent = None;
            let chain = object.group.map(|g| doc.group_chain(g)).unwrap_or_default();
            for group in chain.into_iter().rev() {
                if seen.insert((layer, parent, group)) {
                    tree.children
                        .entry((layer, parent))
                        .or_default()
                        .push(TreeRow {
                            index: i,
                            group: Some(group),
                        });
                }
                tree.members.entry(group).or_default().push(i);
                parent = Some(group);
            }
            tree.children
                .entry((layer, parent))
                .or_default()
                .push(TreeRow {
                    index: i,
                    group: None,
                });
        }
        tree
    }
}
#[cfg(test)]
mod tree_tests {
    use super::*;
    #[test]
    fn tree_keeps_each_leaf_once_and_reconstructs_nested_members() {
        let mut d = Document::demo();
        let count = d.objects.len();
        let selected = editing::group(&mut d, &[0, 1]);
        let child = d.objects[selected[0]].group.unwrap();
        editing::group(&mut d, &(0..count).collect::<Vec<_>>());
        let root = d.root_group(0).unwrap();
        let tree = ObjectTree::new(&d);
        assert_eq!(tree.children[&(1, None)].len(), 1);
        assert_eq!(tree.members[&root].len(), count);
        assert_eq!(tree.members[&child].len(), 2);
        let mut leaves: Vec<_> = tree
            .children
            .values()
            .flatten()
            .filter(|r| r.group.is_none())
            .map(|r| r.index)
            .collect();
        leaves.sort_unstable();
        assert_eq!(leaves, (0..count).collect::<Vec<_>>());
    }
    #[test]
    fn tree_indexes_ten_thousand_leaves_without_duplicate_rows() {
        let mut d = Document::default();
        let object = Document::demo().objects[0].clone();
        for i in 0..10000 {
            let mut o = object.clone();
            o.id = i + 1;
            o.group = Some(i / 2 + 1);
            d.objects.push(o);
        }
        let tree = ObjectTree::new(&d);
        assert_eq!(tree.children[&(1, None)].len(), 5000);
        assert_eq!(tree.members.len(), 5000);
        assert!(tree.members.values().all(|v| v.len() == 2));
        assert_eq!(
            tree.children
                .values()
                .flatten()
                .filter(|r| r.group.is_none())
                .count(),
            10000
        );
    }
}
