//! Stacking commands are planned in root-group units before changing the document.
use crate::{document::Document, editing};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Copy, Debug)]
pub enum Order {
    FrontPage,
    BackPage,
    FrontLayer,
    BackLayer,
    ForwardOne,
    BackOne,
    Reverse,
}
#[derive(Default)]
struct Unit {
    indices: Vec<usize>,
    selected: bool,
}
#[derive(Default)]
pub struct Availability {
    enabled: [bool; 7],
}
impl Availability {
    pub fn allows(&self, mode: Order) -> bool {
        self.enabled[mode as usize]
    }
}
fn build_units(
    doc: &Document,
    selected: &[usize],
) -> Result<(Vec<Vec<Unit>>, Vec<(usize, u64)>), String> {
    let picked: BTreeSet<_> = selected.iter().copied().collect();
    let mut units: Vec<Vec<Unit>> = (0..doc.layers.len()).map(|_| Vec::new()).collect();
    let mut keys: Vec<BTreeMap<(bool, u64), usize>> =
        (0..doc.layers.len()).map(|_| BTreeMap::new()).collect();
    let mut previous = vec![None; doc.layers.len()];
    let before: Vec<_> = doc
        .paint_order()
        .into_iter()
        .map(|i| (i, doc.objects[i].layer))
        .collect();
    for &(i, layer) in &before {
        let l = doc
            .layers
            .iter()
            .position(|v| v.id == layer)
            .ok_or("Camada ausente")?;
        let key = doc.root_group(i).map_or((false, i as u64), |g| (true, g));
        let position = if let Some(&p) = keys[l].get(&key) {
            if previous[l] != Some(key) {
                return Err("Grupo legado intercalado: desagrupe antes de ordenar para preservar a ordem dos demais objetos".into());
            }
            p
        } else {
            let p = units[l].len();
            keys[l].insert(key, p);
            units[l].push(Unit::default());
            p
        };
        previous[l] = Some(key);
        units[l][position].indices.push(i);
        units[l][position].selected |= picked.contains(&i);
    }
    Ok((units, before))
}
/// Inspect the same root units as the planner without cloning a document per command.
pub fn availability(doc: &Document, indices: &[usize]) -> Availability {
    let selected = editing::expand_groups(doc, indices.iter().copied());
    let mut state = Availability::default();
    if selected.is_empty() {
        return state;
    }
    let Ok((units, _)) = build_units(doc, &selected) else {
        return state;
    };
    let editable = |l: usize| doc.layers[l].visible && !doc.layers[l].locked;
    let front_target = (0..units.len()).rev().find(|&l| editable(l));
    let back_target = (0..units.len()).find(|&l| editable(l));
    let mut participating = 0;
    let mut count = 0;
    for (l, layer) in units.iter().enumerate() {
        let Some(first) = layer.iter().position(|u| u.selected) else {
            continue;
        };
        participating += 1;
        count += layer.iter().filter(|u| u.selected).count();
        let last = layer.iter().rposition(|u| u.selected).unwrap();
        let front = layer
            .iter()
            .rposition(|u| !u.selected)
            .is_some_and(|i| i > first);
        let back = layer
            .iter()
            .position(|u| !u.selected)
            .is_some_and(|i| i < last);
        state.enabled[Order::FrontLayer as usize] |= front;
        state.enabled[Order::BackLayer as usize] |= back;
        state.enabled[Order::FrontPage as usize] |= front_target.is_some_and(|t| t != l || front);
        state.enabled[Order::BackPage as usize] |= back_target.is_some_and(|t| t != l || back);
        state.enabled[Order::ForwardOne as usize] |=
            layer.windows(2).any(|p| p[0].selected && !p[1].selected)
                || (layer.last().is_some_and(|u| u.selected) && (l + 1..units.len()).any(editable));
        state.enabled[Order::BackOne as usize] |=
            layer.windows(2).any(|p| !p[0].selected && p[1].selected)
                || (layer.first().is_some_and(|u| u.selected) && (0..l).any(editable));
    }
    state.enabled[Order::Reverse as usize] = participating == 1 && count >= 2;
    state
}
#[derive(Clone, Copy, Debug)]
pub enum Relative {
    InFront,
    Behind,
}
fn relative_selection(doc: &Document, indices: &[usize]) -> Result<Vec<usize>, String> {
    let selected = editing::expand_groups(doc, indices.iter().copied());
    if selected.is_empty() || indices.iter().any(|i| !selected.contains(i)) {
        return Err("Selecione objetos ou grupos inteiros editáveis".into());
    }
    let mut layers = BTreeMap::new();
    for (i, o) in doc.objects.iter().enumerate() {
        if let Some(root) = doc.root_group(i) {
            if layers
                .insert(root, o.layer)
                .is_some_and(|old| old != o.layer)
            {
                return Err(
                    "Grupo legado em várias camadas: desagrupe antes de ordenar por alvo".into(),
                );
            }
        }
    }
    Ok(selected)
}
pub fn can_target(doc: &Document, indices: &[usize]) -> bool {
    let Ok(selected) = relative_selection(doc, indices) else {
        return false;
    };
    let Ok((units, _)) = build_units(doc, &selected) else {
        return false;
    };
    units
        .iter()
        .flatten()
        .any(|u| !u.selected && u.indices.iter().all(|&i| doc.editable(i)))
}
pub fn apply_relative(
    doc: &mut Document,
    indices: &[usize],
    target: usize,
    mode: Relative,
) -> Result<Vec<usize>, String> {
    let selected = relative_selection(doc, indices)?;
    if !doc.editable(target) {
        return Err("Escolha um alvo visível e desbloqueado".into());
    }
    let target_members = editing::expand_groups(doc, [target]);
    if target_members.is_empty() {
        return Err("O grupo alvo contém objetos bloqueados".into());
    }
    if target_members.iter().any(|i| selected.contains(i)) {
        return Err("Escolha um alvo fora da seleção".into());
    }
    let layer = doc
        .layers
        .iter()
        .position(|l| l.id == doc.objects[target].layer)
        .ok_or("Camada alvo ausente")?;
    let (mut units, before) = build_units(doc, &selected)?;
    let mut moving = Vec::new();
    for current in &mut units {
        let mut rest = Vec::new();
        for unit in current.drain(..) {
            if unit.selected {
                moving.push(unit);
            } else {
                rest.push(unit);
            }
        }
        *current = rest;
    }
    let position = units[layer]
        .iter()
        .position(|u| u.indices.contains(&target))
        .ok_or("Alvo não disponível")?
        + usize::from(matches!(mode, Relative::InFront));
    units[layer].splice(position..position, moving);
    finish_plan(doc, &selected, units, before)
}
pub fn apply(doc: &mut Document, indices: &[usize], mode: Order) -> Result<Vec<usize>, String> {
    let selected = editing::expand_groups(doc, indices.iter().copied());
    if selected.is_empty() {
        return Ok(selected);
    }
    let (mut units, before) = build_units(doc, &selected)?;
    let editable = |l: usize| doc.layers[l].visible && !doc.layers[l].locked;
    match mode {
        Order::Reverse => {
            let participating: Vec<_> = units
                .iter()
                .enumerate()
                .filter_map(|(l, u)| u.iter().any(|v| v.selected).then_some(l))
                .collect();
            if participating.len() > 1 {
                return Err("Inverter ordem: selecione unidades de uma mesma camada; inversão entre camadas ainda não disponível".into());
            }
            if let Some(&l) = participating.first() {
                let slots: Vec<_> = units[l]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, u)| u.selected.then_some(i))
                    .collect();
                for i in 0..slots.len() / 2 {
                    units[l].swap(slots[i], slots[slots.len() - 1 - i]);
                }
            }
        }
        Order::FrontPage | Order::BackPage => {
            let front = matches!(mode, Order::FrontPage);
            let target = if front {
                (0..units.len()).rev().find(|&l| editable(l))
            } else {
                (0..units.len()).find(|&l| editable(l))
            };
            let Some(target) = target else {
                return Ok(selected);
            };
            let mut moving = Vec::new();
            for layer in &mut units {
                let mut rest = Vec::new();
                for unit in layer.drain(..) {
                    if unit.selected {
                        moving.push(unit)
                    } else {
                        rest.push(unit)
                    }
                }
                *layer = rest;
            }
            if front {
                units[target].extend(moving);
            } else {
                moving.append(&mut units[target]);
                units[target] = moving;
            }
        }
        Order::FrontLayer | Order::BackLayer => {
            let front = matches!(mode, Order::FrontLayer);
            for layer in &mut units {
                let (mut moving, mut rest): (Vec<_>, Vec<_>) =
                    layer.drain(..).partition(|u| u.selected);
                if front {
                    rest.append(&mut moving);
                    *layer = rest;
                } else {
                    moving.append(&mut rest);
                    *layer = moving;
                }
            }
        }
        Order::ForwardOne | Order::BackOne => {
            let forward = matches!(mode, Order::ForwardOne);
            // Capture edge blocks before local swaps. Incoming blocks never move twice.
            let mut transfers = Vec::new();
            for l in 0..units.len() {
                let count = if forward {
                    units[l].iter().rev().take_while(|u| u.selected).count()
                } else {
                    units[l].iter().take_while(|u| u.selected).count()
                };
                if count == 0 {
                    continue;
                }
                let target = if forward {
                    (l + 1..units.len()).find(|&j| editable(j))
                } else {
                    (0..l).rev().find(|&j| editable(j))
                };
                if let Some(target) = target {
                    let range = if forward {
                        units[l].len() - count..units[l].len()
                    } else {
                        0..count
                    };
                    let moving: Vec<_> = units[l].drain(range).collect();
                    transfers.push((target, moving));
                }
            }
            for layer in &mut units {
                if forward {
                    for i in (0..layer.len().saturating_sub(1)).rev() {
                        if layer[i].selected && !layer[i + 1].selected {
                            layer.swap(i, i + 1);
                        }
                    }
                } else {
                    for i in 0..layer.len().saturating_sub(1) {
                        if !layer[i].selected && layer[i + 1].selected {
                            layer.swap(i, i + 1);
                        }
                    }
                }
            }
            for (target, mut moving) in transfers {
                if forward {
                    moving.append(&mut units[target]);
                    units[target] = moving;
                } else {
                    units[target].extend(moving);
                }
            }
        }
    }
    finish_plan(doc, &selected, units, before)
}
fn finish_plan(
    doc: &mut Document,
    selected: &[usize],
    units: Vec<Vec<Unit>>,
    before: Vec<(usize, u64)>,
) -> Result<Vec<usize>, String> {
    let picked: BTreeSet<_> = selected.iter().copied().collect();
    let mut after = Vec::new();
    for (l, layer) in units.iter().enumerate() {
        for unit in layer {
            for &i in &unit.indices {
                after.push((i, doc.layers[l].id));
            }
        }
    }
    if after == before {
        return Ok(selected.to_vec());
    }
    // A source keeps a slot in its final layer. Project each layer's desired
    // sequence onto those slots; unrelated layers are never canonicalized.
    let mut final_layers = vec![None; doc.objects.len()];
    for &(source, layer) in &after {
        if final_layers[source].replace(layer).is_some() {
            return Err("Plano de ordenação duplicado".into());
        }
    }
    let mut slots: BTreeMap<u64, std::collections::VecDeque<usize>> = BTreeMap::new();
    for (i, layer) in final_layers.into_iter().enumerate() {
        slots
            .entry(layer.ok_or("Plano de ordenação incompleto")?)
            .or_default()
            .push_back(i);
    }
    let mut projected = vec![(0, 0); doc.objects.len()];
    for &(source, layer) in &after {
        let destination = slots
            .get_mut(&layer)
            .and_then(|q| q.pop_front())
            .ok_or("Plano de ordenação inconsistente")?;
        projected[destination] = (source, layer);
    }
    after = projected;
    let mut result = Vec::new();
    let mut objects = Vec::with_capacity(doc.objects.len());
    for (i, layer) in after {
        let mut object = doc.objects[i].clone();
        object.layer = layer;
        if picked.contains(&i) {
            result.push(objects.len());
        }
        objects.push(object);
    }
    let original = std::mem::replace(&mut doc.objects, objects);
    if let Err(error) = doc.validate_groups() {
        doc.objects = original;
        return Err(error);
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{Geometry, Layer, Object, BLUE};
    use vello::kurbo::Rect;
    #[test]
    fn relative_front_and_behind_are_immediate_and_preserve_block_order() {
        for (mode, expected) in [
            (Relative::InFront, vec![2, 3, 1, 4, 5]),
            (Relative::Behind, vec![2, 1, 3, 4, 5]),
        ] {
            let mut d = scene();
            let old = d.clone();
            let target = selected(&d, 3)[0];
            let selection = apply_relative(&mut d, &[0], target, mode).unwrap();
            assert_eq!(ids(&d), expected);
            assert_eq!(d.objects[selection[0]].id, 1);
            assert_only_layers_changed(&d, &old, &[]);
        }
        let mut d = scene();
        apply_relative(&mut d, &[0, 2, 2], 1, Relative::Behind).unwrap();
        assert_eq!(ids(&d), [1, 3, 2, 4, 5]);
    }
    #[test]
    fn relative_cross_layer_selection_preserves_unrelated_slots_and_attributes() {
        let mut d = transfer_scene();
        let old = d.clone();
        let target = selected(&d, 5)[0];
        let selection = apply_relative(&mut d, &[0, 2], target, Relative::Behind).unwrap();
        assert_eq!(ids(&d), [3, 4, 1, 2, 5]);
        assert_eq!(d.objects[1], old.objects[1]);
        assert_eq!(
            selection
                .iter()
                .map(|&i| d.objects[i].id)
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_only_layers_changed(&d, &old, &[(1, 3), (2, 3)]);
    }
    #[test]
    fn relative_nested_source_and_target_move_whole_roots_without_reparenting() {
        let mut d = scene();
        d.objects[0].group = Some(11);
        d.objects[1].group = Some(12);
        d.group_parents.insert(11, 20);
        d.group_parents.insert(12, 20);
        d.objects[2].group = Some(30);
        d.objects[2].layer = 3;
        d.objects[4].group = Some(31);
        d.group_parents.insert(30, 40);
        d.group_parents.insert(31, 40);
        d.objects[0].transform = vello::kurbo::Affine::new([1., 0.2, 0.3, 2., 40., 50.]);
        let old = d.clone();
        apply_relative(&mut d, &[0], 2, Relative::InFront).unwrap();
        assert_eq!(ids(&d), [4, 3, 5, 1, 2]);
        assert_eq!(d.group_parents, old.group_parents);
        assert_only_layers_changed(&d, &old, &[(1, 3), (2, 3)]);
        d.validate_groups().unwrap();
    }
    #[test]
    fn relative_invalid_targets_and_selection_never_mutate_document() {
        let d = scene();
        for (selection, target) in [
            (vec![0], 0),
            (vec![0], 3),
            (vec![0], usize::MAX),
            (vec![], 1),
            (vec![usize::MAX], 1),
            (vec![0, 3], 1),
        ] {
            let mut current = d.clone();
            assert!(apply_relative(&mut current, &selection, target, Relative::Behind).is_err());
            assert_eq!(current, d);
        }
        let mut d = scene();
        d.objects[1].group = Some(22);
        d.objects[2].group = Some(22);
        d.objects[2].locked = true;
        let old = d.clone();
        assert!(apply_relative(&mut d, &[0], 1, Relative::InFront).is_err());
        assert_eq!(d, old);
        d.objects[2].locked = false;
        d.objects[1].visible = false;
        let old = d.clone();
        assert!(apply_relative(&mut d, &[0], 1, Relative::Behind).is_err());
        assert_eq!(d, old);
    }
    #[test]
    fn relative_adjacent_noop_preserves_noncanonical_document_and_availability() {
        let mut d = transfer_scene();
        let old = d.clone();
        let target = selected(&d, 2)[0];
        assert!(can_target(&d, &[0]));
        apply_relative(&mut d, &[0], target, Relative::Behind).unwrap();
        assert_eq!(d, old);
        let source = selected(&d, 3);
        let target = selected(&d, 2)[0];
        apply_relative(&mut d, &source, target, Relative::InFront).unwrap();
        assert_eq!(d, old);
        assert!(!can_target(&d, &[]));
        assert!(!can_target(&d, &[usize::MAX]));
        let all = editing::expand_groups(&d, 0..d.objects.len());
        assert!(!can_target(&d, &all));
    }
    #[test]
    fn relative_legacy_interleaving_and_multilayer_roots_are_refused() {
        let mut d = scene();
        d.objects[0].group = Some(7);
        d.objects[2].group = Some(7);
        let old = d.clone();
        assert!(!can_target(&d, &[0]));
        assert!(apply_relative(&mut d, &[0], 1, Relative::Behind).is_err());
        assert_eq!(d, old);
        let mut d = scene();
        d.objects[0].group = Some(7);
        d.objects[4].group = Some(7);
        let old = d.clone();
        assert!(!can_target(&d, &[1]));
        assert!(apply_relative(&mut d, &[1], 2, Relative::Behind).is_err());
        assert_eq!(d, old);
    }
    #[test]
    fn relative_roundtrip_preserves_pages_and_native_svg_structure() {
        let mut d = transfer_scene();
        d.add_page(false).unwrap();
        d.activate_page(1);
        let old = d.clone();
        let target = selected(&d, 5)[0];
        apply_relative(&mut d, &[0, 2], target, Relative::InFront).unwrap();
        assert_eq!(d.pages[1], old.pages[1]);
        assert_eq!(
            crate::storage::decode(&crate::storage::encode(&d)).unwrap(),
            d
        );
        let mut active = d.clone();
        active.pages.retain(|p| p.id == active.active_page);
        assert_eq!(
            crate::svg::import(&crate::svg::export(&active)).unwrap().0,
            active
        );
    }
    fn scene() -> Document {
        let mut d = Document::default();
        for _ in 0..3 {
            d.add_object(Object::new(
                "r",
                Geometry::Rectangle(Rect::new(0., 0., 10., 10.)),
                Some(BLUE),
                None,
                0.,
            ));
        }
        d.layers.push(Layer {
            id: 2,
            name: "locked".into(),
            visible: true,
            locked: true,
        });
        d.layers.push(Layer {
            id: 3,
            name: "upper".into(),
            visible: true,
            locked: false,
        });
        d.active_layer = 2;
        d.add_object(d.objects[0].clone());
        d.active_layer = 3;
        d.add_object(d.objects[0].clone());
        d.active_layer = 1;
        d
    }
    fn ids(d: &Document) -> Vec<u64> {
        d.paint_order().iter().map(|&i| d.objects[i].id).collect()
    }
    fn selected(d: &Document, id: u64) -> Vec<usize> {
        vec![d.objects.iter().position(|o| o.id == id).unwrap()]
    }
    #[test]
    fn step_inside_layer_then_cross_skips_locked_and_preserves_ids() {
        let mut d = scene();
        let mut s = vec![0];
        s = apply(&mut d, &s, Order::ForwardOne).unwrap();
        assert_eq!(ids(&d), [2, 1, 3, 4, 5]);
        s = apply(&mut d, &s, Order::ForwardOne).unwrap();
        assert_eq!(ids(&d), [2, 3, 1, 4, 5]);
        s = apply(&mut d, &s, Order::ForwardOne).unwrap();
        assert_eq!(d.objects[s[0]].layer, 3);
        assert_eq!(ids(&d), [2, 3, 4, 1, 5]);
        s = apply(&mut d, &s, Order::BackOne).unwrap();
        assert_eq!(d.objects[s[0]].layer, 1);
        assert_eq!(ids(&d), [2, 3, 1, 4, 5]);
    }
    #[test]
    fn selected_block_moves_once_without_reversal() {
        let mut d = scene();
        apply(&mut d, &[0, 1], Order::ForwardOne).unwrap();
        assert_eq!(ids(&d), [3, 1, 2, 4, 5]);
    }
    #[test]
    fn page_and_layer_extrema_are_distinct() {
        let mut d = scene();
        let mut s = apply(&mut d, &[0], Order::FrontLayer).unwrap();
        assert_eq!(d.objects[s[0]].layer, 1);
        s = apply(&mut d, &s, Order::FrontPage).unwrap();
        assert_eq!(ids(&d), [2, 3, 4, 5, 1]);
        assert_eq!(d.objects[s[0]].layer, 3);
        s = apply(&mut d, &s, Order::BackPage).unwrap();
        assert_eq!(ids(&d), [1, 2, 3, 4, 5]);
        assert_eq!(d.objects[s[0]].layer, 1);
    }
    #[test]
    fn boundary_noop_preserves_complete_document() {
        let mut d = scene();
        let s = selected(&d, 5);
        let old = d.clone();
        apply(&mut d, &s, Order::ForwardOne).unwrap();
        assert_eq!(d, old);
        let s = selected(&d, 1);
        apply(&mut d, &s, Order::BackOne).unwrap();
        assert_eq!(d, old);
        apply(&mut d, &s, Order::BackLayer).unwrap();
        assert_eq!(d, old);
    }
    #[test]
    fn group_transfers_as_one_unit_with_tree_and_attributes() {
        let mut d = scene();
        let mut group = editing::group(&mut d, &[0, 1]);
        group.push(d.objects.iter().position(|o| o.id == 3).unwrap());
        editing::group(&mut d, &group);
        let groups = d.group_parents.clone();
        let all = editing::expand_groups(&d, 0..d.objects.len());
        let s: Vec<_> = all
            .into_iter()
            .filter(|&i| d.objects[i].layer == 1)
            .collect();
        let old: BTreeMap<_, _> = d
            .objects
            .iter()
            .map(|o| (o.id, (o.geometry.clone(), o.transform, o.fill)))
            .collect();
        let s = apply(&mut d, &s, Order::FrontPage).unwrap();
        assert_eq!(s.len(), 3);
        assert!(s.iter().all(|&i| d.objects[i].layer == 3));
        assert_eq!(d.group_parents, groups);
        for o in &d.objects {
            assert_eq!((o.geometry.clone(), o.transform, o.fill), old[&o.id]);
        }
        assert!(d.validate_groups().is_ok());
    }
    #[test]
    fn transfer_can_enter_empty_layer_and_skips_hidden_layers() {
        let mut d = scene();
        d.layers[2].visible = false;
        d.layers.push(Layer {
            id: 4,
            name: "empty".into(),
            visible: true,
            locked: false,
        });
        let s = apply(&mut d, &[2], Order::ForwardOne).unwrap();
        assert_eq!(d.objects[s[0]].layer, 4);
    }
    #[test]
    fn legacy_interleaved_group_is_rejected_without_mutation() {
        let mut d = scene();
        d.objects[0].group = Some(77);
        d.objects[2].group = Some(77);
        let old = d.clone();
        assert!(apply(&mut d, &[0], Order::FrontPage).is_err());
        assert_eq!(d, old);
    }
    #[test]
    fn incoming_unit_crosses_only_one_editable_layer_per_command() {
        let mut d = scene();
        d.layers[1].locked = false;
        let s = apply(&mut d, &[2], Order::ForwardOne).unwrap();
        assert_eq!(d.objects[s[0]].layer, 2);
        assert_eq!(ids(&d), [1, 2, 3, 4, 5]);
    }
    #[test]
    fn reverse_selected_slots_preserves_others_and_is_an_involution() {
        let mut d = scene();
        let old = d.clone();
        let selection = apply(&mut d, &[0, 2, 2, usize::MAX], Order::Reverse).unwrap();
        assert_eq!(ids(&d), [3, 2, 1, 4, 5]);
        assert_eq!(d.objects[1], old.objects[1]);
        assert_eq!(d.objects[3..], old.objects[3..]);
        assert_eq!(selection, [0, 2]);
        for o in &d.objects {
            assert_eq!(o, old.objects.iter().find(|v| v.id == o.id).unwrap());
        }
        apply(&mut d, &selection, Order::Reverse).unwrap();
        assert_eq!(d, old);
    }
    #[test]
    fn reverse_nested_root_preserves_leaf_order_tree_and_full_attributes() {
        let mut d = scene();
        d.objects[0].group = Some(11);
        d.objects[1].group = Some(12);
        d.group_parents.insert(11, 20);
        d.group_parents.insert(12, 20);
        d.objects[0].transform = vello::kurbo::Affine::new([1., 0.2, 0.3, 2., 40., 50.]);
        d.objects[1].fill = Some([9, 70, 210, 128]);
        d.validate_groups().unwrap();
        let old = d.clone();
        assert!(availability(&d, &[0, 2]).allows(Order::Reverse));
        let selection = apply(&mut d, &[0, 2], Order::Reverse).unwrap();
        assert_eq!(ids(&d), [3, 1, 2, 4, 5]);
        assert_eq!(selection, [0, 1, 2]);
        assert_eq!(d.group_parents, old.group_parents);
        for o in &d.objects {
            assert_eq!(o, old.objects.iter().find(|v| v.id == o.id).unwrap());
        }
        apply(&mut d, &selection, Order::Reverse).unwrap();
        assert_eq!(d, old);
    }
    #[test]
    fn reverse_unavailable_cross_layer_is_rejected_without_mutation() {
        let mut d = scene();
        let old = d.clone();
        assert!(!availability(&d, &[0, 4]).allows(Order::Reverse));
        assert!(apply(&mut d, &[0, 4], Order::Reverse)
            .unwrap_err()
            .contains("mesma camada"));
        assert_eq!(d, old);
        // A locked layer is never silently added to the selected cohort.
        assert!(!availability(&d, &[0, 3]).allows(Order::Reverse));
        apply(&mut d, &[0, 3], Order::Reverse).unwrap();
        assert_eq!(d, old);
    }
    #[test]
    fn reverse_empty_or_one_root_is_neutral_and_legacy_interleaving_is_refused() {
        let mut d = scene();
        let old = d.clone();
        for selection in [&[][..], &[0][..], &[usize::MAX][..]] {
            assert!(!availability(&d, selection).allows(Order::Reverse));
            apply(&mut d, selection, Order::Reverse).unwrap();
            assert_eq!(d, old);
        }
        d.objects[0].group = Some(11);
        d.objects[1].group = Some(11);
        assert!(!availability(&d, &[0, 1]).allows(Order::Reverse));
        let old = d.clone();
        apply(&mut d, &[0], Order::Reverse).unwrap();
        assert_eq!(d, old);
        d.objects[1].group = None;
        d.objects[2].group = Some(11);
        let old = d.clone();
        assert!(apply(&mut d, &[0, 1], Order::Reverse).is_err());
        assert_eq!(d, old);
    }
    #[test]
    fn availability_matches_planner_for_selection_masks_and_unsupported_groups() {
        let mut nested = transfer_scene();
        nested.objects[0].group = Some(11);
        nested.objects[2].group = Some(12);
        nested.group_parents.insert(11, 20);
        nested.group_parents.insert(12, 20);
        let mut legacy = scene();
        legacy.objects[0].group = Some(77);
        legacy.objects[2].group = Some(77);
        let mut locked = scene();
        for layer in &mut locked.layers {
            layer.locked = true;
        }
        for original in [scene(), transfer_scene(), nested, legacy, locked] {
            for mask in 0..32 {
                let selection: Vec<_> = (0..5).filter(|i| mask & (1 << i) != 0).collect();
                let state = availability(&original, &selection);
                for mode in [
                    Order::FrontPage,
                    Order::BackPage,
                    Order::FrontLayer,
                    Order::BackLayer,
                    Order::ForwardOne,
                    Order::BackOne,
                    Order::Reverse,
                ] {
                    let mut d = original.clone();
                    let expected = apply(&mut d, &selection, mode).is_ok() && d != original;
                    assert_eq!(state.allows(mode), expected, "mask={mask}, mode={mode:?}");
                }
            }
        }
    }
    fn transfer_scene() -> Document {
        let mut d = interleaved_scene();
        d.layers[2].locked = false;
        d
    }
    fn assert_only_layers_changed(d: &Document, before: &Document, expected: &[(u64, u64)]) {
        assert_eq!(d.layers, before.layers);
        assert_eq!(d.group_parents, before.group_parents);
        for object in &d.objects {
            let mut old = before
                .objects
                .iter()
                .find(|v| v.id == object.id)
                .unwrap()
                .clone();
            if let Some(&(_, layer)) = expected.iter().find(|&&(id, _)| id == object.id) {
                old.layer = layer;
            }
            assert_eq!(object, &old);
        }
    }
    #[test]
    fn transfer_up_preserves_unrelated_layer_slots_and_full_attributes() {
        let mut d = transfer_scene();
        let before = d.clone();
        let selection = selected(&d, 3);
        let selection = apply(&mut d, &selection, Order::ForwardOne).unwrap();
        assert_eq!(ids(&d), [1, 2, 4, 3, 5]);
        assert_eq!(
            d.objects.iter().map(|o| o.id).collect::<Vec<_>>(),
            [1, 4, 2, 3, 5]
        );
        assert_eq!(d.objects[1], before.objects[1]);
        assert_eq!(d.objects[selection[0]].id, 3);
        assert_only_layers_changed(&d, &before, &[(3, 3)]);
    }
    #[test]
    fn transfer_down_preserves_unrelated_layer_slots_and_full_attributes() {
        let mut d = transfer_scene();
        let before = d.clone();
        let selection = selected(&d, 5);
        let selection = apply(&mut d, &selection, Order::BackOne).unwrap();
        assert_eq!(ids(&d), [1, 2, 3, 5, 4]);
        assert_eq!(
            d.objects.iter().map(|o| o.id).collect::<Vec<_>>(),
            [1, 4, 2, 3, 5]
        );
        assert_eq!(d.objects[1], before.objects[1]);
        assert_eq!(d.objects[selection[0]].id, 5);
        assert_only_layers_changed(&d, &before, &[(5, 1)]);
    }
    #[test]
    fn transfer_to_empty_layer_does_not_reorder_other_layers() {
        let mut d = transfer_scene();
        d.layers[2].visible = false;
        d.layers.push(Layer {
            id: 4,
            name: "empty".into(),
            visible: true,
            locked: false,
        });
        let before = d.clone();
        let selection = selected(&d, 3);
        let selection = apply(&mut d, &selection, Order::ForwardOne).unwrap();
        assert_eq!(
            d.objects.iter().map(|o| o.id).collect::<Vec<_>>(),
            [1, 4, 2, 5, 3]
        );
        assert_eq!(d.objects[selection[0]].layer, 4);
        assert_only_layers_changed(&d, &before, &[(3, 4)]);
    }
    #[test]
    fn transfer_nested_root_preserves_uninvolved_slots_and_leaf_order() {
        let mut d = transfer_scene();
        d.objects[0].group = Some(11);
        d.objects[2].group = Some(12);
        d.group_parents.insert(11, 20);
        d.group_parents.insert(12, 20);
        d.objects[0].transform = vello::kurbo::Affine::new([1., 0.2, 0.3, 2., 40., 50.]);
        let before = d.clone();
        let selection = apply(&mut d, &[0], Order::FrontPage).unwrap();
        assert_eq!(ids(&d), [3, 4, 5, 1, 2]);
        assert_eq!(d.objects[4], before.objects[4]);
        assert_eq!(d.objects[1], before.objects[1]);
        assert_eq!(
            selection
                .iter()
                .map(|&i| d.objects[i].id)
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_only_layers_changed(&d, &before, &[(1, 3), (2, 3)]);
        d.validate_groups().unwrap();
        let encoded = crate::storage::encode(&d);
        assert_eq!(crate::storage::decode(&encoded).unwrap(), d);
    }
    fn interleaved_scene() -> Document {
        let mut d = scene();
        let objects = d.objects.clone();
        d.objects = [0, 3, 1, 4, 2]
            .into_iter()
            .map(|i| objects[i].clone())
            .collect();
        // The upper layer is locked only to constrain page extrema to layer 1.
        d.layers[2].locked = true;
        d
    }
    fn assert_nontransferring_pairs(mut d: Document) {
        for (forward, backward, expected) in [
            (Order::ForwardOne, Order::BackOne, vec![2, 1, 3, 4, 5]),
            (Order::FrontLayer, Order::BackLayer, vec![2, 3, 1, 4, 5]),
            (Order::FrontPage, Order::BackPage, vec![2, 3, 1, 4, 5]),
        ] {
            let original = d.clone();
            let xml = crate::storage::encode(&d);
            let selection = selected(&d, 1);
            let selection = apply(&mut d, &selection, forward).unwrap();
            if original.objects[0].group.is_none() {
                assert_eq!(ids(&d), expected);
            }
            assert_eq!(
                d.objects[1], original.objects[1],
                "locked layer global slot"
            );
            assert_eq!(d.objects[3], original.objects[3], "upper layer global slot");
            assert_eq!(d.group_parents, original.group_parents);
            for object in &d.objects {
                assert_eq!(
                    object,
                    original.objects.iter().find(|v| v.id == object.id).unwrap()
                );
            }
            assert!(selection.iter().all(|&i| d.objects[i].layer == 1));
            assert_eq!(d.objects[selection[0]].id, 1);
            apply(&mut d, &selection, backward).unwrap();
            assert_eq!(d, original, "inverse pair {forward:?}/{backward:?}");
            assert_eq!(crate::storage::encode(&d), xml);
        }
    }
    #[test]
    fn nontransferring_commands_preserve_interleaved_storage_and_saved_state() {
        assert_nontransferring_pairs(interleaved_scene());
    }
    #[test]
    fn nontransferring_group_commands_preserve_interleaved_slots_and_attributes() {
        let mut d = interleaved_scene();
        d.objects[0].group = Some(11);
        d.objects[2].group = Some(12);
        d.group_parents.insert(11, 20);
        d.group_parents.insert(12, 20);
        d.objects[0].transform = vello::kurbo::Affine::new([1., 0.2, 0.3, 2., 40., 50.]);
        d.objects[2].fill = Some([20, 60, 180, 128]);
        d.validate_groups().unwrap();
        assert_nontransferring_pairs(d);
    }
    #[test]
    fn reverse_twice_preserves_noncanonical_cross_layer_storage_exactly() {
        let mut d = scene();
        let objects = d.objects.clone();
        d.objects = [0, 3, 1, 4, 2]
            .into_iter()
            .map(|i| objects[i].clone())
            .collect();
        let old = d.clone();
        assert_eq!(ids(&d), [1, 2, 3, 4, 5]);
        let selection = apply(&mut d, &[0, 4], Order::Reverse).unwrap();
        assert_eq!(ids(&d), [3, 2, 1, 4, 5]);
        assert_eq!(selection, [0, 4]);
        assert_eq!(d.objects[1], old.objects[1]);
        assert_eq!(d.objects[2], old.objects[2]);
        assert_eq!(d.objects[3], old.objects[3]);
        apply(&mut d, &selection, Order::Reverse).unwrap();
        assert_eq!(d, old);
        // A neutral operation also leaves a noncanonical document byte-for-byte serializable.
        let xml = crate::storage::encode(&d);
        apply(&mut d, &[0], Order::Reverse).unwrap();
        assert_eq!(crate::storage::encode(&d), xml);
    }
}
