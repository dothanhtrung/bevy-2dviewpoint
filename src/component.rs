// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, Image, Resource, Timer};
use bevy::sprite::TextureAtlasLayout;
pub use bevy_2dviewangle_macro::ActorsTexturesCollection;

pub trait ActorsTexturesCollection {
    fn get_all(
        &self,
    ) -> Vec<(
        Option<u64>,
        Option<u16>,
        Option<Angle>,
        Option<&Handle<Image>>,
        Option<&Handle<TextureAtlasLayout>>,
    )>;
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Angle {
    Any,
    #[default]
    Front,
    Back,
    Left,
    Right,
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
}

#[derive(Default, Clone)]
pub struct ViewSprite {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Default, Deref, DerefMut)]
pub struct ViewTextures(HashMap<Angle, ViewSprite>);

#[derive(Component, Default)]
pub struct DynamicActor {
    pub angle: Angle,
    pub action: u16,
    pub actor: u64,
    pub flipped: bool,
    pub animation_timer: Option<Timer>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ActorsTextures(HashMap<u64, HashMap<u16, ViewTextures>>);

#[derive(Event)]
pub struct ViewChanged {
    pub entity: Entity,
}

impl ViewTextures {
    pub fn from(items: Vec<(Angle, ViewSprite)>) -> Self {
        let mut map = HashMap::new();
        for (key, value) in items {
            map.insert(key, value);
        }
        Self(map)
    }
}

impl ActorsTextures {
    pub fn load_asset_loader<T: ActorsTexturesCollection>(&mut self, loader: &T) {
        let mut actor_id = 0;
        let mut action_id = 0;
        for (actor, action, angle, image, atlas_layout) in loader.get_all() {
            actor_id = actor.unwrap_or(actor_id);
            action_id = action.unwrap_or(action_id);
            let field_angle = angle.unwrap_or_default();
            let actor;
            if let Some(_actor) = self.get_mut(&actor_id) {
                actor = _actor;
            } else {
                self.insert(actor_id, HashMap::default());
                actor = self.get_mut(&actor_id).unwrap();
            }

            let action;
            if let Some(_action) = actor.get_mut(&action_id) {
                action = _action;
            } else {
                actor.insert(action_id, ViewTextures::default());
                action = actor.get_mut(&action_id).unwrap();
            }

            let any = action.get(&Angle::Any).cloned();
            let sprite;
            if let Some(_sprite) = action.get_mut(&field_angle) {
                sprite = _sprite;
            } else {
                action.insert(field_angle, ViewSprite::default());
                sprite = action.get_mut(&field_angle).unwrap();
            }

            if let Some(image_handle) = image {
                sprite.image = Some(image_handle.clone());
            } else if any.is_some() {
                sprite.image = any.as_ref().unwrap().image.clone();
            }

            if let Some(atlas_layout_handle) = atlas_layout {
                sprite.layout = Some(atlas_layout_handle.clone());
            } else if any.is_some() {
                sprite.layout = any.unwrap().layout.clone();
            }

            if field_angle == Angle::Any {
                let any = sprite.clone();
                for s in action.values_mut() {
                    if s.image.is_none() {
                        s.image = any.image.clone();
                    }
                    if s.layout.is_none() {
                        s.layout = any.layout.clone();
                    }
                }
            }
        }
    }
}