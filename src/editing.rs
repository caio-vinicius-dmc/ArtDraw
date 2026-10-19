use crate::document::{Document, Geometry, Node, Object};
use std::collections::{BTreeMap, BTreeSet};
use vello::kurbo::{Affine, Point, Rect, Vec2};
#[derive(Clone, Copy)]
pub enum Align {
    Left,
    CenterX,
    Right,
    Top,
    CenterY,
    Bottom,
}
pub fn bounds(doc: &Document, indices: &[usize]) -> Option<Rect> {
    indices
        .iter()
        .filter_map(|&i| doc.objects.get(i))
        .map(Object::bounds)
        .reduce(|a, b| a.union(b))
}
pub fn expand_groups(doc: &Document, indices: impl IntoIterator<Item = usize>) -> Vec<usize> {
    let mut units: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for i in 0..doc.objects.len() {
        if let Some(root) = doc.root_group(i) {
            units.entry(root).or_default().push(i);
        }
    }
    let mut result = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for i in indices {
        if !doc.editable(i) {
            continue;
        }
        if let Some(root) = doc.root_group(i) {
            if visited.insert(root) {
                let members = &units[&root];
                if members.iter().all(|&j| doc.editable(j)) {
                    result.extend(members.iter().copied());
                }
            }
        } else {
            result.insert(i);
        }
    }
    result.into_iter().collect()
}
pub fn marquee(doc: &Document, rect: Rect, crossing: bool) -> Vec<usize> {
    let mut units: BTreeMap<(bool, u64), Vec<usize>> = BTreeMap::new();
    for (i, _) in doc.objects.iter().enumerate() {
        units
            .entry(doc.root_group(i).map_or((false, i as u64), |g| (true, g)))
            .or_default()
            .push(i);
    }
    let mut picked = Vec::new();
    for indices in units.values() {
        if !indices.iter().all(|&i| doc.editable(i)) {
            continue;
        }
        let b = bounds(doc, indices).unwrap();
        let inside = if crossing {
            b.x0 <= rect.x1 && b.x1 >= rect.x0 && b.y0 <= rect.y1 && b.y1 >= rect.y0
        } else {
            rect.x0 <= b.x0 && rect.y0 <= b.y0 && rect.x1 >= b.x1 && rect.y1 >= b.y1
        };
        if inside {
            picked.extend(indices.iter().copied());
        }
    }
    picked.sort_unstable();
    picked
}
pub fn transform(doc: &mut Document, indices: &[usize], affine: Affine) {
    for &i in indices {
        if doc.editable(i) {
            doc.objects[i].transform = affine * doc.objects[i].transform;
        }
    }
}
pub fn align(doc: &mut Document, indices: &[usize], mode: Align, page: bool) {
    let Some(b) = bounds(doc, indices) else {
        return;
    };
    let reference = if page {
        Rect::new(0., 0., doc.width, doc.height)
    } else {
        b
    };
    let mut units: BTreeMap<(bool, u64), Vec<usize>> = BTreeMap::new();
    for &i in indices {
        units
            .entry(doc.root_group(i).map_or((false, i as u64), |g| (true, g)))
            .or_default()
            .push(i);
    }
    for unit in units.values() {
        let r = bounds(doc, unit).unwrap();
        let delta = match mode {
            Align::Left => Vec2::new(reference.x0 - r.x0, 0.),
            Align::CenterX => Vec2::new(reference.center().x - r.center().x, 0.),
            Align::Right => Vec2::new(reference.x1 - r.x1, 0.),
            Align::Top => Vec2::new(0., reference.y0 - r.y0),
            Align::CenterY => Vec2::new(0., reference.center().y - r.center().y),
            Align::Bottom => Vec2::new(0., reference.y1 - r.y1),
        };
        transform(doc, unit, Affine::translate(delta));
    }
}
pub fn distribute(doc: &mut Document, indices: &[usize], horizontal: bool) {
    let mut map: BTreeMap<(bool, u64), Vec<usize>> = BTreeMap::new();
    for &i in indices {
        map.entry(doc.root_group(i).map_or((false, i as u64), |g| (true, g)))
            .or_default()
            .push(i);
    }
    let mut units: Vec<_> = map.into_values().collect();
    if units.len() < 3 {
        return;
    }
    let coordinate = |doc: &Document, unit: &Vec<usize>| {
        let c = bounds(doc, unit).unwrap().center();
        if horizontal {
            c.x
        } else {
            c.y
        }
    };
    units.sort_by(|a, b| coordinate(doc, a).total_cmp(&coordinate(doc, b)));
    let first = coordinate(doc, &units[0]);
    let last = coordinate(doc, units.last().unwrap());
    let step = (last - first) / (units.len() - 1) as f64;
    for (k, unit) in units.iter().enumerate().skip(1).take(units.len() - 2) {
        let delta = first + step * k as f64 - coordinate(doc, unit);
        transform(
            doc,
            unit,
            Affine::translate(if horizontal {
                Vec2::new(delta, 0.)
            } else {
                Vec2::new(0., delta)
            }),
        );
    }
}
/// Insert a captured tree, assigning independent group/object identities on the active page.
pub fn insert_objects(
    doc: &mut Document,
    objects: &[Object],
    parents: &BTreeMap<u64, u64>,
) -> Vec<usize> {
    let mut used = BTreeSet::new();
    for object in objects {
        let mut group = object.group;
        while let Some(g) = group {
            if !used.insert(g) {
                break;
            }
            group = parents.get(&g).copied();
        }
    }
    let mut occupied: BTreeSet<_> = doc
        .objects
        .iter()
        .filter_map(|o| o.group)
        .chain(doc.group_parents.keys().copied())
        .chain(doc.group_parents.values().copied())
        .collect();
    let mut next = doc.next_group();
    let map: BTreeMap<_, _> = used
        .into_iter()
        .map(|g| {
            while occupied.contains(&next) {
                next = next.checked_add(1).unwrap_or(1);
            }
            let id = next;
            occupied.insert(id);
            next = next.checked_add(1).unwrap_or(1);
            (g, id)
        })
        .collect();
    for (&child, &parent) in parents {
        if let (Some(&c), Some(&p)) = (map.get(&child), map.get(&parent)) {
            doc.group_parents.insert(c, p);
        }
    }
    objects
        .iter()
        .map(|object| {
            let mut o = object.clone();
            o.group = o.group.map(|g| map[&g]);
            doc.add_object(o)
        })
        .collect()
}
pub fn duplicate(
    doc: &mut Document,
    objects: &[Object],
    parents: &BTreeMap<u64, u64>,
    offset: Vec2,
) -> Vec<usize> {
    let copies: Vec<_> = objects
        .iter()
        .map(|object| {
            let mut o = object.clone();
            o.locked = false;
            o.visible = true;
            o.translate(offset);
            o.name.push_str(" cópia");
            o
        })
        .collect();
    insert_objects(doc, &copies, parents)
}
pub fn group(doc: &mut Document, indices: &[usize]) -> Vec<usize> {
    let indices = expand_groups(doc, indices.iter().copied());
    let units: BTreeSet<_> = indices
        .iter()
        .map(|&i| doc.root_group(i).map_or((false, i as u64), |g| (true, g)))
        .collect();
    if units.len() < 2 {
        return indices;
    }
    if indices
        .iter()
        .filter_map(|&i| doc.objects[i].group)
        .any(|g| doc.group_chain(g).len() >= 64)
    {
        return indices;
    }
    let new_group = doc.next_group();
    let layer = doc.objects[indices[0]].layer;
    for (is_group, id) in units {
        if is_group {
            doc.group_parents.insert(id, new_group);
        } else {
            doc.objects[id as usize].group = Some(new_group);
        }
    }
    for &i in &indices {
        doc.objects[i].layer = layer;
    }
    reorder(doc, &indices, true)
}
pub fn ungroup(doc: &mut Document, indices: &[usize], all: bool) {
    let indices = expand_groups(doc, indices.iter().copied());
    let roots: BTreeSet<_> = indices.iter().filter_map(|&i| doc.root_group(i)).collect();
    for i in indices {
        if all || doc.objects[i].group.is_some_and(|g| roots.contains(&g)) {
            doc.objects[i].group = None;
        }
    }
    doc.group_parents
        .retain(|_, parent| !roots.contains(parent));
    doc.prune_groups();
}
pub fn reorder(doc: &mut Document, indices: &[usize], front: bool) -> Vec<usize> {
    let picked: BTreeSet<_> = indices.iter().copied().collect();
    let mut selected = Vec::new();
    let mut rest = Vec::new();
    for (i, o) in doc.objects.drain(..).enumerate() {
        if picked.contains(&i) {
            selected.push(o)
        } else {
            rest.push(o)
        }
    }
    let start = if front { rest.len() } else { 0 };
    let count = selected.len();
    if front {
        rest.extend(selected);
        doc.objects = rest
    } else {
        selected.extend(rest);
        doc.objects = selected
    }
    (start..start + count).collect()
}
pub fn polygon(rect: Rect, count: usize, star: bool) -> Geometry {
    let n = count.clamp(3, 64);
    let c = rect.center();
    let mut nodes = Vec::new();
    for k in 0..(if star { 2 * n } else { n }) {
        let angle = -std::f64::consts::FRAC_PI_2
            + std::f64::consts::TAU * k as f64 / (if star { 2 * n } else { n }) as f64;
        let r = if star && k % 2 == 1 { 0.45 } else { 1. };
        nodes.push(Node::corner(Point::new(
            c.x + angle.cos() * rect.width() * 0.5 * r,
            c.y + angle.sin() * rect.height() * 0.5 * r,
        )));
    }
    Geometry::Path {
        nodes,
        closed: true,
    }
}
pub fn spiral(rect: Rect, turns: usize) -> Geometry {
    let mut nodes = Vec::new();
    let count = turns.clamp(1, 20) * 48;
    let c = rect.center();
    for k in 0..=count {
        let t = k as f64 / count as f64;
        let angle = t * std::f64::consts::TAU * turns as f64;
        nodes.push(Node::corner(Point::new(
            c.x + t * angle.cos() * rect.width() / 2.,
            c.y + t * angle.sin() * rect.height() / 2.,
        )));
    }
    Geometry::Path {
        nodes,
        closed: false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{Layer, BLUE};
    fn scene() -> Document {
        let mut d = Document::default();
        for x in [0., 30., 100.] {
            d.add_object(Object::new(
                "r",
                Geometry::Rectangle(Rect::new(x, 0., x + 10., 10.)),
                Some(BLUE),
                None,
                0.,
            ));
        }
        d
    }
    #[test]
    fn distribution_centers_are_equal() {
        let mut d = scene();
        distribute(&mut d, &[0, 1, 2], true);
        assert_eq!(d.objects[1].bounds().center().x, 55.);
    }
    #[test]
    fn group_alignment_preserves_relative_position() {
        let mut d = scene();
        d.objects[0].group = Some(1);
        d.objects[1].group = Some(1);
        let delta = d.objects[1].bounds().x0 - d.objects[0].bounds().x0;
        align(&mut d, &[0, 1, 2], Align::CenterX, true);
        assert_eq!(d.objects[1].bounds().x0 - d.objects[0].bounds().x0, delta);
    }
    #[test]
    fn layer_lock_and_visibility_affect_hit() {
        let mut d = scene();
        d.layers[0].locked = true;
        assert_eq!(d.hit(Point::new(5., 5.), 1.), None);
        d.layers[0].locked = false;
        d.objects[0].visible = false;
        assert!(!d.visible(0));
        assert_eq!(d.hit(Point::new(5., 5.), 1.), None);
    }
    #[test]
    fn groups_do_not_transform_partially_when_locked() {
        let mut d = scene();
        d.objects[0].group = Some(1);
        d.objects[1].group = Some(1);
        d.objects[1].locked = true;
        assert!(expand_groups(&d, [0]).is_empty());
    }
    #[test]
    fn copying_remaps_group_ids() {
        let mut d = scene();
        d.objects[0].group = Some(1);
        d.objects[1].group = Some(1);
        let original = d.objects[..2].to_vec();
        let ids = duplicate(&mut d, &original, &BTreeMap::new(), Vec2::new(10., 20.));
        assert_ne!(d.objects[ids[0]].group, Some(1));
        assert_eq!(d.objects[ids[0]].group, d.objects[ids[1]].group);
    }
    #[test]
    fn layer_order_is_independent_of_indices() {
        let mut d = scene();
        d.layers.push(Layer {
            id: 2,
            name: "top".into(),
            visible: true,
            locked: false,
        });
        d.objects[0].layer = 2;
        assert_eq!(d.paint_order(), vec![1, 2, 0]);
    }
    #[test]
    fn marquee_excludes_invisible() {
        let mut d = scene();
        d.objects[1].visible = false;
        assert_eq!(
            marquee(&d, Rect::new(-1., -1., 200., 20.), false),
            vec![0, 2]
        );
    }
    #[test]
    fn mirror_twice_restores_geometry() {
        let mut d = scene();
        let before = d.clone();
        let c = bounds(&d, &[0, 1]).unwrap().center();
        let t = Affine::translate(c.to_vec2())
            * Affine::scale_non_uniform(-1., 1.)
            * Affine::translate(-c.to_vec2());
        transform(&mut d, &[0, 1], t);
        transform(&mut d, &[0, 1], t);
        assert_eq!(d, before);
    }
    #[test]
    fn polygon_and_star_have_expected_nodes() {
        assert!(
            matches!(polygon(Rect::new(0.,0.,100.,100.),5,true),Geometry::Path{nodes,closed:true}if nodes.len()==10)
        );
    }

    #[test]
    fn marquee_contains_whole_group() {
        let mut d = scene();
        d.objects[0].group = Some(1);
        d.objects[1].group = Some(1);
        assert!(marquee(&d, Rect::new(-1., -1., 20., 20.), false).is_empty());
        assert_eq!(marquee(&d, Rect::new(-1., -1., 20., 20.), true), vec![0, 1]);
    }
    #[test]
    fn crossing_selects_zero_height_line() {
        let mut d = Document::default();
        d.add_object(Object::new(
            "line",
            Geometry::Path {
                nodes: vec![
                    Node::corner(Point::new(0., 5.)),
                    Node::corner(Point::new(100., 5.)),
                ],
                closed: false,
            },
            None,
            Some(BLUE),
            1.,
        ));
        assert_eq!(marquee(&d, Rect::new(40., 0., 60., 10.), true), vec![0]);
    }
    #[test]
    fn collective_scale_uses_common_anchor() {
        let mut d = scene();
        transform(&mut d, &[0, 1], Affine::scale_non_uniform(2., 3.));
        assert_eq!(bounds(&d, &[0, 1]), Some(Rect::new(0., 0., 80., 30.)));
        assert_eq!(d.objects[2].bounds(), Rect::new(100., 0., 110., 10.));
    }
    #[test]
    fn all_alignment_modes_respect_page() {
        for (mode, x, y) in [
            (Align::Left, 5., 5.),
            (Align::CenterX, 397., 5.),
            (Align::Right, 789., 5.),
            (Align::Top, 5., 5.),
            (Align::CenterY, 5., 561.5),
            (Align::Bottom, 5., 1118.),
        ] {
            let mut d = scene();
            align(&mut d, &[0], mode, true);
            let c = d.objects[0].bounds().center();
            assert!((c.x - x).abs() < 1e-8 && (c.y - y).abs() < 1e-8);
        }
    }
    #[test]
    fn reorder_preserves_selected_and_unselected_order() {
        let mut d = scene();
        let original = d.objects.clone();
        let ids = reorder(&mut d, &[0, 2], true);
        assert_eq!(ids, vec![1, 2]);
        assert_eq!(
            d.objects,
            vec![
                original[1].clone(),
                original[0].clone(),
                original[2].clone()
            ]
        );
    }
    fn nested_scene() -> Document {
        let mut d = scene();
        let ids = group(&mut d, &[0, 1]);
        let subgroup = d.objects[ids[0]].group.unwrap();
        group(&mut d, &[0, 1, 2]);
        assert!(d.group_parents.contains_key(&subgroup));
        d
    }
    #[test]
    fn nesting_preserves_children_and_does_not_wrap_one_unit() {
        let mut d = nested_scene();
        assert_eq!(d.group_parents.len(), 1);
        assert_eq!(expand_groups(&d, [0]), vec![0, 1, 2]);
        let old = d.clone();
        group(&mut d, &[1]);
        assert_eq!(d, old);
        assert!(d.validate_groups().is_ok());
    }
    #[test]
    fn ungroup_one_level_preserves_subgroups_all_removes_them() {
        let mut d = nested_scene();
        let geometry: Vec<_> = d.objects.iter().map(|o| o.transform).collect();
        let subgroup = *d.group_parents.keys().next().unwrap();
        ungroup(&mut d, &[0], false);
        assert!(d.group_parents.is_empty());
        assert_eq!(
            d.objects
                .iter()
                .filter(|o| o.group == Some(subgroup))
                .count(),
            2
        );
        assert_eq!(d.objects.iter().filter(|o| o.group.is_none()).count(), 1);
        assert_eq!(
            d.objects.iter().map(|o| o.transform).collect::<Vec<_>>(),
            geometry
        );
        let mut all = nested_scene();
        ungroup(&mut all, &[0], true);
        assert!(all.group_parents.is_empty() && all.objects.iter().all(|o| o.group.is_none()));
    }
    #[test]
    fn nested_copy_across_pages_keeps_hierarchy_independent() {
        let mut d = nested_scene();
        let objects = d.objects.clone();
        let parents = d.group_parents.clone();
        let ids = duplicate(&mut d, &objects, &parents, Vec2::new(12., 13.));
        assert_eq!(d.group_parents.len(), 2);
        assert_ne!(d.root_group(ids[0]), d.root_group(0));
        assert_eq!(expand_groups(&d, [ids[1]]), ids);
        d.add_page(false).unwrap();
        let imported = insert_objects(&mut d, &objects, &parents);
        assert_eq!(imported, vec![0, 1, 2]);
        assert_eq!(d.group_parents.len(), 1);
        for (new, old) in d.objects.iter().zip(objects) {
            assert_eq!(new.geometry, old.geometry);
            assert_eq!(new.transform, old.transform);
            assert_ne!(new.id, old.id);
        }
        assert!(d.validate_groups().is_ok());
    }
    #[test]
    fn locked_descendant_blocks_entire_nested_unit_and_marquee() {
        let mut d = nested_scene();
        d.objects[2].locked = true;
        assert!(expand_groups(&d, [0]).is_empty());
        assert!(marquee(&d, Rect::new(-100., -100., 300., 300.), true).is_empty());
        let old = d.clone();
        group(&mut d, &[0, 1, 2]);
        assert_eq!(d, old);
    }
    #[test]
    fn align_and_transform_nested_group_preserve_internal_offsets() {
        let mut d = nested_scene();
        let before: Vec<_> = d.objects.iter().map(Object::bounds).collect();
        align(&mut d, &[0, 1, 2], Align::CenterX, true);
        let delta = d.objects[0].bounds().center() - before[0].center();
        for (o, b) in d.objects.iter().zip(before) {
            assert!((o.bounds().center() - b.center() - delta).hypot() < 1e-9);
        }
        let old = d.clone();
        transform(&mut d, &[0, 1, 2], Affine::scale_non_uniform(2., 3.));
        for (new, old) in d.objects.iter().zip(&old.objects) {
            assert_eq!(
                new.transform,
                Affine::scale_non_uniform(2., 3.) * old.transform
            );
        }
    }
    #[test]
    fn hierarchy_validation_rejects_cycles_orphans_depth_and_cross_layer() {
        let mut d = nested_scene();
        let root = d.root_group(0).unwrap();
        let child = *d.group_parents.keys().next().unwrap();
        d.group_parents.insert(root, child);
        assert!(d.validate_groups().is_err());
        d = nested_scene();
        d.group_parents.insert(800, 801);
        assert!(d.validate_groups().is_err());
        d = nested_scene();
        d.objects[0].layer = 77;
        assert!(d.validate_groups().is_err());
        d = scene();
        d.objects[0].group = Some(1);
        for id in 1..64 {
            d.group_parents.insert(id, id + 1);
        }
        assert!(d.validate_groups().is_ok());
        d.group_parents.insert(64, 65);
        assert!(d.validate_groups().is_err());
    }
    #[test]
    fn copied_groups_do_not_collide_when_id_space_wraps() {
        let mut d = scene();
        d.objects[0].group = Some(u64::MAX);
        d.objects[1].group = Some(1);
        d.objects[2].group = Some(3);
        let objects = d.objects[..2].to_vec();
        let old: BTreeSet<_> = d.objects.iter().filter_map(|o| o.group).collect();
        let ids = duplicate(&mut d, &objects, &BTreeMap::new(), Vec2::ZERO);
        let copied: BTreeSet<_> = ids.iter().filter_map(|&i| d.objects[i].group).collect();
        assert_eq!(copied.len(), 2);
        assert!(copied.is_disjoint(&old));
        assert!(d.validate_groups().is_ok());
    }
}
