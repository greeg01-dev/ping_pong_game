use bevy::prelude::{Input, KeyCode, Query, Res, Time, Transform};
use crate::entities::{Paddle1p, Paddle2p};

// function to move the paddle for 1p
pub fn move_paddle_1p(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_1p_query: Query<(&Paddle1p, &mut Transform)>
) {
    for (_, mut transform) in paddle_1p_query.iter_mut() {
        // move paddle up when W key is pressed
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y = {
                (transform.translation.y + time.delta_seconds() * 400.0)
                    .min(200.0)
            };
        }
        // move paddle down when S key is pressed
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y = {
                (transform.translation.y - time.delta_seconds() * 400.0)
                    .max(-200.0)
            }
        }
    }
}

// function to move the paddle for 2p
pub fn move_paddle_2p(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_2p_query: Query<(&Paddle2p, &mut Transform)>
) {
    for (_, mut transform) in paddle_2p_query.iter_mut() {
        // move paddle up when Up key is pressed
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y = {
                (transform.translation.y + time.delta_seconds() * 400.0)
                    .min(200.0)
            };
        }
        // move paddle down when Down key is pressed
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y = {
                (transform.translation.y - time.delta_seconds() * 400.0)
                    .max(-200.0)
            }
        }
    }
}