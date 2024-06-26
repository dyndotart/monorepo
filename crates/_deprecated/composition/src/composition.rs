use bevy_app::App;
#[cfg(feature = "interactive")]
use bevy_ecs::{bundle::Bundle, entity::Entity, query::With};
#[cfg(feature = "interactive")]
use bevy_hierarchy::BuildWorldChildren;
use dyn_bevy_render_skeleton::RenderPlugin;

use crate::modules::composition::CompositionPlugin;

#[cfg(feature = "interactive")]
use super::modules::node::components::{
    bundles::{FrameNodeBundle, NodeBundle, PaintBundle, RectangleNodeBundle, TextNodeBundle},
    types::Root,
};

use super::{dtif::DTIFComposition, events::input_event::InputEvent, modules::node::NodePlugin};

pub struct Composition {
    app: App,
}

impl Composition {
    pub fn new(dtif: DTIFComposition) -> Self {
        let mut app = App::new();

        // Register plugins
        app.add_plugins((RenderPlugin, NodePlugin, CompositionPlugin { dtif }));
        #[cfg(feature = "interactive")]
        app.add_plugins(super::modules::interactive_composition::InteractiveCompositionPlugin);

        return Self { app };
    }

    // =========================================================================
    // Getter & Setter
    // =========================================================================

    pub fn get_app(&self) -> &App {
        &self.app
    }

    pub fn get_app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    // =========================================================================
    // Lifecycle
    // =========================================================================

    pub fn update(&mut self) {
        self.app.update();
    }

    // =========================================================================
    // Spawn
    // =========================================================================

    // TODO: Try to solve with 'NodeCreated' event as its basically the same logic,
    //  but ofc it can't directly return the created Entity
    //  -> Solve with callback id where the Entity is sent back under the callback id or so

    #[cfg(feature = "interactive")]
    pub fn spawn_node_bundle(
        &mut self,
        bundle: NodeBundle,
        maybe_parent_id: Option<Entity>,
    ) -> Entity {
        use crate::modules::node::components::bundles::VectorNodeBundle;

        let maybe_fill_mixin = match &bundle {
            NodeBundle::Rectangle(RectangleNodeBundle { fill_mixin, .. })
            | NodeBundle::Frame(FrameNodeBundle { fill_mixin, .. })
            | NodeBundle::Text(TextNodeBundle { fill_mixin, .. })
            | NodeBundle::Vector(VectorNodeBundle { fill_mixin, .. }) => Some(fill_mixin.clone()),
            _ => None,
        };
        let entity_id = match bundle {
            NodeBundle::Frame(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            NodeBundle::Rectangle(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            NodeBundle::Group(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            NodeBundle::Text(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            NodeBundle::Vector(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            NodeBundle::Polygon(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            NodeBundle::Ellipse(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            NodeBundle::Star(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
        };

        if let Some(fill_mixin) = maybe_fill_mixin {
            if let Some(mut entity) = self.app.world.get_entity_mut(entity_id) {
                entity.push_children(&fill_mixin.paint_ids);
            }
        }

        return entity_id;
    }

    #[cfg(feature = "interactive")]
    pub fn spawn_paint_bundle(
        &mut self,
        bundle: PaintBundle,
        maybe_parent_id: Option<Entity>,
    ) -> Entity {
        let entity_id = match bundle {
            PaintBundle::Solid(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            PaintBundle::Image(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
            PaintBundle::Gradient(bundle) => self.spawn_bundle(bundle, maybe_parent_id),
        };

        return entity_id;
    }

    #[cfg(feature = "interactive")]
    pub fn spawn_bundle<B: Bundle + std::fmt::Debug>(
        &mut self,
        bundle: B,
        maybe_parent_id: Option<Entity>,
    ) -> Entity {
        let entity = self.app.world.spawn::<B>(bundle).id();

        // If no parent id provided the root node will become the parent
        let maybe_parent_entity = maybe_parent_id.or_else(|| {
            self.app
                .world
                .query_filtered::<Entity, With<Root>>()
                .iter(&self.app.world)
                .next()
        });

        // Establish potential parent child relation
        if let Some(parent_entity) = maybe_parent_entity {
            if let Some(mut parent) = self.app.world.get_entity_mut(parent_entity) {
                parent.push_children(&[entity]);
            }
        }

        return entity;
    }

    // =========================================================================
    // Events
    // =========================================================================

    pub fn register_events<T: InputEvent>(&mut self, events: Vec<T>) {
        for event in events {
            self.register_event(event);
        }
    }

    pub fn register_event<T: InputEvent>(&mut self, event: T) {
        event.send_to_ecs(&mut self.app.world);
    }

    // =========================================================================
    // Other
    // =========================================================================

    pub fn clear(&mut self) {
        self.app.world.clear_all();
    }
}
