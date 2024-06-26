use bevy_ecs::entity::Entity;
use dyn_composition::{
    modules::node::components::types::{
        GradientPaintVariant, LinearGradientPaintTransform, RadialGradientPaintTransform,
    },
    utils::continuous_id::ContinuousId,
};

use crate::{
    events::output_event::ElementChangeEvent,
    mixin_change::PaintMixinChange,
    resources::{
        changed_components::ChangedPaint,
        svg_composition::{
            svg_bundle::{BaseSVGBundle, SVGBundle},
            svg_element::{
                attributes::{SVGAttribute, SVGMeasurementUnit, SVGUnitsVariant},
                mapper::map_blend_mode,
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
            svg_node::ElementReference,
            SVGCompositionRes,
        },
    },
};

use super::{utils::rgb_to_hex, SVGPaint};

#[derive(Debug)]
pub struct GradientSVGPaint {
    bundle: BaseSVGBundle,
    variant: GradientSVGPaintVariant,

    defs: ElementReference,

    // Paint elements
    paint_gradient: ElementReference,
    paint_gradient_stops: ElementReference,
    paint_rect: ElementReference,
}

#[derive(Debug)]
pub enum GradientSVGPaintVariant {
    Linear,
    Radial,
}

impl SVGBundle for GradientSVGPaint {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }

    fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        self.bundle.drain_changes()
    }

    fn to_string(&self, composition: &SVGCompositionRes) -> String {
        self.bundle.to_string(composition)
    }
}

impl SVGPaint for GradientSVGPaint {
    fn apply_paint_change(
        &mut self,
        changed_paint: &ChangedPaint,
        id_generator: &mut ContinuousId,
    ) {
        for change in &changed_paint.changes {
            match change {
                PaintMixinChange::GradientPaint(mixin) => {
                    match &mixin.variant {
                        GradientPaintVariant::Linear { transform } => match transform {
                            LinearGradientPaintTransform::Internal { start, end, .. } => {
                                self.bundle
                                    .get_child_element_mut(self.paint_gradient.index)
                                    .unwrap()
                                    .set_attributes(vec![
                                        SVGAttribute::X1 { x1: start.x },
                                        SVGAttribute::Y1 { y1: start.y },
                                        SVGAttribute::X2 { x2: end.x },
                                        SVGAttribute::Y2 { y2: end.y },
                                    ]);
                            }
                            _ => {}
                        },
                        GradientPaintVariant::Radial { transform } => match transform {
                            RadialGradientPaintTransform::Internal {
                                center,
                                radius,
                                rotation,
                            } => {
                                // TODO
                            }
                            _ => {}
                        },
                    }
                }
                PaintMixinChange::GradientStopsMixin(mixin) => {
                    let paint_gradient_id = self
                        .bundle
                        .get_child_element_mut(self.paint_gradient.index)
                        .unwrap()
                        .get_id();

                    let elements = self
                        .bundle
                        .get_child_portal_mut(self.paint_gradient_stops.index)
                        .unwrap();

                    // Remove old gradient stop elements
                    elements.iter_mut().for_each(|element| element.remove());

                    // Add new gradient stop elements
                    for gradient_stop in &mixin.gradient_stops {
                        let mut gradient_stop_element = SVGElement::new(SVGTag::Stop, id_generator);
                        gradient_stop_element.set_attributes(vec![
                            SVGAttribute::Offset {
                                offset: gradient_stop.position,
                            },
                            SVGAttribute::StopColor {
                                stop_color: rgb_to_hex(gradient_stop.color),
                            },
                        ]);
                        gradient_stop_element.append_to_parent(paint_gradient_id);
                        elements.push(gradient_stop_element);
                    }
                }
                PaintMixinChange::PaintComposition(mixin) => {
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
                PaintMixinChange::Dimension(mixin) => {
                    self.bundle
                        .get_child_element_mut(self.paint_rect.index)
                        .unwrap()
                        .set_attributes(vec![
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
                PaintMixinChange::Blend(mixin) => {
                    let root_element = self.bundle.get_root_mut();
                    root_element.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    root_element.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                _ => {}
            }
        }
    }
}

impl GradientSVGPaint {
    pub fn new(
        entity: Entity,
        variant: GradientSVGPaintVariant,
        id_generator: &mut ContinuousId,
    ) -> Self {
        // Create root element
        let mut element = SVGElement::new(SVGTag::Group, id_generator);
        #[cfg(feature = "tracing")]
        element.set_attribute(SVGAttribute::Name {
            name: GradientSVGPaint::create_element_name(
                element.get_id(),
                String::from("root"),
                false,
            ),
        });
        let mut bundle = BaseSVGBundle::new(element, entity);

        let mut defs_element = SVGElement::new(SVGTag::Defs, id_generator);
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SVGAttribute::Name {
            name: GradientSVGPaint::create_element_name(defs_id, String::from("defs"), false),
        });
        let defs_index = bundle.append_child_element(defs_element);

        // Create paint elements
        let mut paint_gradient_element = SVGElement::new(
            match variant {
                GradientSVGPaintVariant::Linear => SVGTag::LinearGradient,
                GradientSVGPaintVariant::Radial => SVGTag::RadialGradient,
            },
            id_generator,
        );
        let paint_gradient_id = paint_gradient_element.get_id();
        #[cfg(feature = "tracing")]
        paint_gradient_element.set_attribute(SVGAttribute::Name {
            name: GradientSVGPaint::create_element_name(
                paint_gradient_id,
                String::from("paint-gradient"),
                false,
            ),
        });
        paint_gradient_element.set_attribute(SVGAttribute::GradientUnits {
            gradient_units: SVGUnitsVariant::UserSpaceOnUse,
        });
        let paint_gradient_index = bundle
            .append_child_element_to(defs_index, paint_gradient_element)
            .unwrap();

        let paint_gradient_stops_index =
            bundle.append_child_portal_to(paint_gradient_index).unwrap();

        let mut paint_rect_element = SVGElement::new(SVGTag::Rect, id_generator);
        #[cfg(feature = "tracing")]
        paint_rect_element.set_attribute(SVGAttribute::Name {
            name: GradientSVGPaint::create_element_name(
                paint_rect_id,
                String::from("paint-rect"),
                false,
            ),
        });
        paint_rect_element.set_attribute(SVGAttribute::ReferencedFill {
            id: paint_gradient_id,
        });
        let paint_rect_index = bundle.append_child_element(paint_rect_element);

        Self {
            bundle,
            variant,
            defs: ElementReference { index: defs_index },

            // Paint element references
            paint_gradient: ElementReference {
                index: paint_gradient_index,
            },
            paint_gradient_stops: ElementReference {
                index: paint_gradient_stops_index,
            },
            paint_rect: ElementReference {
                index: paint_rect_index,
            },
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("image-fill_{}_{}{}", category, id, def_part)
    }
}
