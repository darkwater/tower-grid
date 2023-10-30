use bevy::prelude::*;
use bevy_tnua::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{cursor::PlayerCursor, Player};
use crate::{hud::build_menu::BuildMenu, world_tile::WorldTile};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Move,
    Jump,
    ContextMenu,
}

pub fn manager_bundle() -> InputManagerBundle<Action> {
    InputManagerBundle::<Action> {
        action_state: ActionState::default(),
        input_map: InputMap::default()
            .insert(
                VirtualDPad {
                    up: KeyCode::W.into(),
                    down: KeyCode::S.into(),
                    left: KeyCode::A.into(),
                    right: KeyCode::D.into(),
                },
                Action::Move,
            )
            .insert(KeyCode::Space, Action::Jump)
            .insert(MouseButton::Right, Action::ContextMenu)
            .build(),
    }
}

pub fn move_player(
    mut player: Query<(&mut TnuaController, &ActionState<Action>), With<Player>>,
    // time: Res<Time>,
) {
    for (mut controller, action_state) in player.iter_mut() {
        let mut axis_pair = action_state.axis_pair(Action::Move).unwrap();
        axis_pair.clamp_length(1.);

        let direction = Vec3::new(axis_pair.x(), 0., -axis_pair.y());

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction * Player::WALK_SPEED,
            desired_forward: direction,

            // Must be larger than the height of the entity's center from the bottom of its
            // collider, or else the character will not float and Tnua will not work properly:
            float_height: Player::HEIGHT / 2. + 0.1,

            acceleration: 80.,

            // TnuaBuiltinWalk has many other fields that can be configured:
            ..Default::default()
        });

        if action_state.pressed(Action::Jump) {
            // The jump action must be fed as long as the player holds the button.
            controller.action(TnuaBuiltinJump {
                // The full height of the jump, if the player does not release the button:
                height: 4.0,

                // TnuaBuiltinJump too has other fields that can be configured:
                ..Default::default()
            });
        }
    }
}

pub fn hud_actions(
    player: Query<&ActionState<Action>, With<Player>>,
    build_menu: Query<Entity, With<BuildMenu>>,
    cursor: Query<&PlayerCursor>,
    mut commands: Commands,
) {
    for action_state in player.iter() {
        if action_state.just_pressed(Action::ContextMenu) {
            let cursor = cursor.get_single().unwrap();

            if let Some(ref raycast) = cursor.raycast {
                commands.spawn(BuildMenu::new(
                    cursor.screen_pos,
                    WorldTile::from_vec3(raycast.1.point),
                ));
            }
        }

        if action_state.just_released(Action::ContextMenu) {
            if let Ok(build_menu) = build_menu.get_single() {
                commands.entity(build_menu).despawn_recursive();
            }
        }
    }
}
