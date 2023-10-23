use bevy::prelude::*;
use bevy_tnua::prelude::*;
use leafwing_input_manager::prelude::*;

use super::Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Move,
    Jump,
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
            .build(),
    }
}

pub fn move_player(
    mut query: Query<(&Player, &mut TnuaController, &ActionState<Action>)>,
    // time: Res<Time>,
) {
    for (player, mut controller, action_state) in query.iter_mut() {
        let mut axis_pair = action_state.axis_pair(Action::Move).unwrap();
        axis_pair.clamp_length(1.);

        let direction = Vec3::new(axis_pair.x(), 0., -axis_pair.y());

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction * player.walk_speed,
            desired_forward: direction,

            // Must be larger than the height of the entity's center from the bottom of its
            // collider, or else the character will not float and Tnua will not work properly:
            float_height: 1.,

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

// pub fn move_player(
//     mut player: Query<(&mut KinematicCharacterController, &ActionState<Action>), With<Player>>,
//     time: Res<Time>,
// ) {
//     let (mut kcc, action_state) = player.single_mut();

//     let translation = kcc.translation.get_or_insert_default();

//     if action_state.pressed(Action::Move) {
//         let mut axis_pair = action_state.axis_pair(Action::Move).unwrap();
//         axis_pair.clamp_length(1.);

//         *translation += Vec3::new(
//             axis_pair.x() * time.delta_seconds() * 5.,
//             0.,
//             axis_pair.y() * time.delta_seconds() * 5.,
//         );
//     }

//     if action_state.just_pressed(Action::Jump) {
//         translation.y += 1.;
//     }
// }
