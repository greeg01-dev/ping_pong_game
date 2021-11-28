use bevy::prelude::{Query, Res, Time, Transform};
use crate::entities::Ball;

// function to move the ball
pub fn move_ball(
    time: Res<Time>,
    mut ball_query: Query<(&Ball, &mut Transform)>
) {
    // move the ball
    for (ball, mut ball_transform) in ball_query.iter_mut() {
        ball_transform.translation.x += f32::sin(ball.degree * std::f32::consts::PI / 180.0) * time.delta_seconds() * ball.speed * 100.0;
        ball_transform.translation.y += f32::cos(ball.degree * std::f32::consts::PI / 180.0) * time.delta_seconds() * ball.speed * 100.0;
    }
}