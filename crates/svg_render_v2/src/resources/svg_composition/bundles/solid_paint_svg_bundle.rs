use std::collections::BTreeMap;

use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    mixin_change::MixinChange,
    resources::{
        changed_entities::{ChangedEntity, ChangedEntityType},
        svg_composition::{
            svg_bundle::SVGBundle,
            svg_context::SVGContext,
            svg_element::{
                attributes::{SVGAttribute, SVGMeasurementUnit},
                mapper::map_mat3_to_svg_transform,
                SVGElement, SVGTag,
            },
        },
    },
};

#[derive(Debug)]
pub struct SolidPaintSVGBundle {
    entity: Entity,

    root: SVGElement,
    paint_rect: SVGElement,
}

impl SVGBundle for SolidPaintSVGBundle {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_type(&self) -> ChangedEntityType {
        ChangedEntityType::ShapeNode
    }

    fn append_child(&mut self, svg_bundle: &mut Box<dyn SVGBundle>) -> () {
        // Do nothing as a paint can't have children
    }

    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext) -> () {
        for change in &changed_entity.changes {
            match change {
                MixinChange::Dimension(mixin) => {
                    self.paint_rect.set_attributes(vec![
                        SVGAttribute::Width {
                            width: mixin.width,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                        SVGAttribute::Height {
                            height: mixin.height,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                    ]);
                }
                MixinChange::RelativeTransform(mixin) => {
                    self.root.set_attribute(SVGAttribute::Transform {
                        transform: map_mat3_to_svg_transform(&mixin.relative_transform.0),
                    });
                }
                _ => {}
            }
        }
    }

    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.paint_rect.get_id(), &self.paint_rect);
        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.paint_rect.get_id(), &mut self.paint_rect);
        return children;
    }

    fn get_root_element(&self) -> &SVGElement {
        return &self.root;
    }

    fn get_root_element_mut(&mut self) -> &mut SVGElement {
        return &mut self.root;
    }

    fn to_string(&self, cx: &SVGContext) -> String {
        self.get_root_element().to_string(self, cx)
    }
}

impl SolidPaintSVGBundle {
    pub fn new(entity: Entity, cx: &mut SVGContext) -> Self {
        let mut root_element = cx.create_element(SVGTag::Group);

        // Create  elements

        let mut paint_rect = cx.create_element(SVGTag::Rect);
        root_element.append_child_in_bundle_context(entity, &mut paint_rect);

        Self {
            entity,
            root: root_element,
            paint_rect: paint_rect,
        }
    }
}
