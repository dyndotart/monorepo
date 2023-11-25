use bevy_ecs::entity::Entity;

use crate::core::{
    events::output_event::RenderUpdateEvent,
    mixin_change::MixinChange,
    modules::svg_render::resources::{
        changed_components::ChangedNode,
        svg_composition::{
            svg_bundle::{BaseSVGBundle, SVGBundle},
            svg_element::{
                attributes::{SVGAttribute, SVGMeasurementUnit},
                helper::{construct_svg_path, mat3_to_svg_transform},
                mapper::map_blend_mode,
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
            SVGCompositionRes,
        },
    },
};

use super::{ElementReference, SVGNode};

#[derive(Debug)]
pub struct ShapeSVGNode {
    bundle: BaseSVGBundle,

    // Fill elements
    fill_clip_path: ElementReference,
    fill_clip_path_defs: ElementReference,
    fill_clipped_shape: ElementReference,
    fill_wrapper: ElementReference,
}

impl SVGBundle for ShapeSVGNode {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }
}

impl SVGNode for ShapeSVGNode {
    fn apply_node_change(&mut self, changed_node: &ChangedNode) {
        for change in &changed_node.changes {
            match change {
                MixinChange::Dimension(mixin) => {
                    self.bundle.get_root_mut().set_attributes(vec![
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
                    self.bundle.get_root_mut().set_attributes(vec![
                        (SVGAttribute::Transform {
                            transform: mat3_to_svg_transform(mixin.relative_transform.0),
                        }),
                    ]);
                }
                MixinChange::Path(mixin) => self
                    .bundle
                    .get_child_mut(self.fill_clipped_shape.index)
                    .unwrap()
                    .set_attributes(vec![SVGAttribute::D {
                        d: construct_svg_path(&mixin.vertices),
                    }]),
                MixinChange::Blend(mixin) => {
                    let root_element = self.bundle.get_root_mut();
                    root_element.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    root_element.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                MixinChange::Composition(mixin) => {
                    self.bundle
                        .get_root_mut()
                        .set_styles(vec![SVGStyle::Display {
                            display: if mixin.is_visible {
                                SVGDisplayStyle::Block
                            } else {
                                SVGDisplayStyle::None
                            },
                        }]);
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    fn get_child_append_id(&self) -> Option<&ElementReference> {
        None
    }

    fn get_paint_append_id(&self) -> Option<&ElementReference> {
        Some(&self.fill_wrapper)
    }

    fn drain_updates(&mut self) -> Vec<RenderUpdateEvent> {
        self.get_bundle_mut().drain_updates()
    }

    fn to_string(&self, composition: &SVGCompositionRes) -> String {
        self.bundle.to_string(composition)
    }
}

impl ShapeSVGNode {
    pub fn new(entity: Entity) -> Self {
        // Create root element
        let mut element = SVGElement::new(SVGTag::Group);
        let element_id = element.get_id();
        #[cfg(feature = "trace")]
        element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        });
        let mut bundle = BaseSVGBundle::new(element, entity);

        // Create fill elements
        let mut fill_clip_path_defs = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_defs_id = fill_clip_path_defs.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_defs.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clip_path_defs_id,
                String::from("fill-defs"),
                false,
            ),
        });
        let fill_clip_path_defs_index = bundle.append_child(fill_clip_path_defs);

        let mut fill_clip_path_element = SVGElement::new(SVGTag::ClipPath);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clip_path_id,
                String::from("fill-clip"),
                true,
            ),
        });
        let fill_clip_path_index = bundle
            .append_child_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_shape_element = SVGElement::new(SVGTag::Path);
        let fill_clipped_shape_id = fill_clipped_shape_element.get_id();
        #[cfg(feature = "trace")]
        fill_clipped_shape_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clipped_shape_id,
                String::from("fill-clipped-shape"),
                false,
            ),
        });
        let fill_clipped_shape_index = bundle
            .append_child_to(fill_clip_path_index, fill_clipped_shape_element)
            .unwrap();

        let mut fill_wrapper_element = SVGElement::new(SVGTag::Group);
        let fill_wrapper_id = fill_wrapper_element.get_id();
        #[cfg(feature = "trace")]
        fill_wrapper_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(fill_wrapper_id, String::from("fill"), false),
        });
        fill_wrapper_element.set_attribute(SVGAttribute::ClipPath {
            clip_path: fill_clip_path_id,
        });
        let fill_wrapper_index = bundle.append_child(fill_wrapper_element);

        Self {
            bundle,

            // Fill element references
            fill_clip_path_defs: ElementReference {
                id: fill_clip_path_defs_id,
                index: fill_clip_path_defs_index,
            },
            fill_clip_path: ElementReference {
                id: fill_clip_path_id,
                index: fill_clip_path_index,
            },
            fill_clipped_shape: ElementReference {
                id: fill_clipped_shape_id,
                index: fill_clipped_shape_index,
            },
            fill_wrapper: ElementReference {
                id: fill_wrapper_id,
                index: fill_wrapper_index,
            },
        }
    }

    #[cfg(feature = "trace")]
    fn create_element_name(id: u32, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("shape_{}_{}{}", category, id, def_part)
    }
}
