// use std::time::Duration;

// use bevy::prelude::*;

// pub fn every(duration: Duration) -> impl Condition<()> {
//     IntoSystem::into_system(move |mut last_trigger: Local<Duration>, time: Res<Time>| {
//         if time.elapsed() - duration > *last_trigger {
//             *last_trigger = time.elapsed();
//             true
//         } else {
//             false
//         }
//     })
// }

// pub fn on_key_press(key: KeyCode) -> impl Condition<()> {
//     IntoSystem::into_system(move |input: Res<Input<KeyCode>>| input.just_pressed(key))
// }
