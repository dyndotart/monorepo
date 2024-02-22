use std::{collections::BTreeMap, fmt::Debug};

use super::svg_element::{SvgElement, SvgElementId};
use bevy_ecs::component::Component;

#[cfg(feature = "output_events")]
use super::svg_element::element_changes::SvgElementChanges;

pub trait SvgNode: Debug + Component {
    /// Retrieves child SVG elements in a sorted order, starting from the top-level element and
    /// proceeding hierarchically to its children & siblings.
    ///
    /// Returns a `BTreeMap` mapping `ContinuousId` to references of `SvgElement`,
    /// ensuring the elements are sorted from the highest in the hierarchy to the lowest
    /// while allowing easy querying for single elements.
    fn get_child_elements(&self) -> BTreeMap<SvgElementId, &SvgElement>;

    /// Similar to `get_child_elements`, but returns mutable references to the Svg elements.
    fn get_child_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement>;

    /// Returns a reference to the root `SvgElement`.
    fn get_root_element(&self) -> &SvgElement;

    /// Returns a mutable reference to the root `SvgElement`.
    fn get_root_element_mut(&mut self) -> &mut SvgElement;

    #[cfg(feature = "output_events")]
    fn drain_changes(&mut self) -> Vec<SvgElementChanges> {
        let mut drained_changes: Vec<SvgElementChanges> = Vec::new();

        // Drain changes from root element
        let root = self.get_root_element_mut();
        let changes = root.drain_changes();
        if !changes.is_empty() {
            drained_changes.push(SvgElementChanges {
                id: root.get_id(),
                changes,
            });
        }

        // Drain changes from children
        for (_, child_element) in self.get_child_elements_mut() {
            let changes = child_element.drain_changes();
            if !changes.is_empty() {
                drained_changes.push(SvgElementChanges {
                    id: child_element.get_id(),
                    changes,
                });
            }
        }

        return drained_changes;
    }
}
