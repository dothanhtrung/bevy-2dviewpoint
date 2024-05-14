use bevy::asset::Handle;
use bevy::input::ButtonInput;
use bevy::prelude::{Entity, EventWriter, Image, KeyCode, Query, Res, TextureAtlasLayout};

use bevy_2dviewangle::{ActorsTexturesCollection, Angle, DynamicActor, ViewChanged};

// Struct to load spritesheet
#[derive(ActorsTexturesCollection, Default)]
pub struct MyAssets {
    #[textureview(actor = "frog", action = "idle", angle = "front")]
    pub idle_front: Handle<Image>,

    // If not specify actor/action, the previous value will be used
    #[textureview(angle = "back")]
    pub idle_back: Handle<Image>,

    #[textureview(angle = "left")]
    pub idle_left: Handle<Image>,

    // If angle is any, other angle which has not been defined will use this value
    #[textureview(angle = "any")]
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut actors: Query<(&mut DynamicActor, Entity)>,
    mut action_event: EventWriter<ViewChanged>,
) {
    for (mut act, e) in actors.iter_mut() {
        let mut action = act.action;
        let mut direction = act.angle;

        // Update action id and direction of actor
        if kb_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            action = Action::Idle as u16;
            direction = Angle::Left;
        } else if kb_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            action = Action::Idle as u16;
            direction = Angle::Right;
        } else if kb_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            action = Action::Idle as u16;
            direction = Angle::Back;
        } else if kb_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            action = Action::Idle as u16;
            direction = Angle::Front;
        }

        if action != act.action || direction != act.angle {
            act.action = action;
            act.angle = direction;
            // Send event to change to sprite sheet to another view
            action_event.send(ViewChanged { entity: e });
        }
    }
}