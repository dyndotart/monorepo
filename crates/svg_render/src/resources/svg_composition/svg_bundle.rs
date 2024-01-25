use bevy_ecs::entity::Entity;

use crate::events::output_event::ElementChangeEvent;

use super::{
    svg_element::{SVGChildElementIdentifier, SVGElement},
    SVGCompositionRes,
};

pub trait SVGBundle {
    fn get_bundle(&self) -> &BaseSVGBundle;
    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle;
    fn drain_changes(&mut self) -> Vec<ElementChangeEvent>;
    fn to_string(&self, composition: &SVGCompositionRes) -> String;
}

/// Wrapped SVGElement with static children (known from compile time) for quick access.
#[derive(Debug)]
pub struct BaseSVGBundle {
    entity: Entity,
    // The primary SVG element associated with this bundle
    element: SVGElement,
    // Children that are directly related to this bundles's context
    // whose order doesn't change.
    // Using a Vector for child_elements as:
    // - The size is known at compile time, minimizing dynamic changes
    // - Offers efficient O(1) access by index, suitable for this use case
    // - More memory-efficient and simpler than a HashMap for fixed-size collections
    child_elements: Vec<BundleChildSVGElement>,
}

#[derive(Debug)]
pub enum BundleChildSVGElement {
    Item(SVGElement),
    Collection(Vec<SVGElement>),
}

impl BaseSVGBundle {
    pub fn new(mut element: SVGElement, entity: Entity) -> Self {
        element.define_as_bundle_root(entity);
        Self {
            entity,
            element,
            child_elements: Vec::new(),
        }
    }

    // =========================================================================
    // Getter & Setter
    // =========================================================================

    pub fn get_children(&self) -> &Vec<BundleChildSVGElement> {
        &self.child_elements
    }

    pub fn get_root(&self) -> &SVGElement {
        &self.element
    }

    pub fn get_root_mut(&mut self) -> &mut SVGElement {
        &mut self.element
    }

    pub fn get_child_item(&self, index: usize) -> Option<&SVGElement> {
        let maybe_child = self.child_elements.get(index);
        if let Some(child) = maybe_child {
            match child {
                BundleChildSVGElement::Item(item) => {
                    return Some(item);
                }
                _ => {}
            }
        }
        return None;
    }

    pub fn get_child_item_mut(&mut self, index: usize) -> Option<&mut SVGElement> {
        let maybe_child = self.child_elements.get_mut(index);
        if let Some(child) = maybe_child {
            match child {
                BundleChildSVGElement::Item(item) => {
                    return Some(item);
                }
                _ => {}
            }
        }
        return None;
    }

    // =========================================================================
    // Children
    // =========================================================================

    pub fn append_child_to(&mut self, index: usize, mut element: SVGElement) -> Option<usize> {
        let next_index = self.get_next_child_index();
        if let Some(target_element) = self.child_elements.get_mut(index) {
            if let BundleChildSVGElement::Item(target_element) = target_element {
                target_element.append_child(
                    &mut element,
                    SVGChildElementIdentifier::InBundleContext(self.entity, next_index),
                );
                self.child_elements
                    .push(BundleChildSVGElement::Item(element));
                return Some(next_index);
            }
        }
        return None;
    }

    pub fn append_child(&mut self, mut element: SVGElement) -> usize {
        let next_index = self.get_next_child_index();
        self.element.append_child(
            &mut element,
            SVGChildElementIdentifier::InBundleContext(self.entity, next_index),
        );
        self.child_elements
            .push(BundleChildSVGElement::Item(element));
        return next_index;
    }

    #[inline]
    pub fn get_next_child_index(&self) -> usize {
        self.child_elements.len()
    }

    // =========================================================================
    // Other
    // =========================================================================

    pub fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        let mut drained_updates = Vec::new();

        // Drain updates from root element
        drained_updates.push(ElementChangeEvent {
            id: self.element.get_id(),
            changes: self.element.drain_changes(),
        });

        // Drain updates from child elements
        for child in &mut self.child_elements {
            match child {
                BundleChildSVGElement::Item(child_element) => {
                    let changes = child_element.drain_changes();
                    if !changes.is_empty() {
                        drained_updates.push(ElementChangeEvent {
                            id: child_element.get_id(),
                            changes,
                        })
                    }
                }
                BundleChildSVGElement::Collection(child_elements) => {
                    for child_element in child_elements {
                        let changes = child_element.drain_changes();
                        if !changes.is_empty() {
                            drained_updates.push(ElementChangeEvent {
                                id: child_element.get_id(),
                                changes,
                            })
                        }
                    }
                }
            }
        }

        return drained_updates;
    }

    pub fn to_string(&self, composition: &SVGCompositionRes) -> String {
        self.element.to_string(self, composition)
    }
}
