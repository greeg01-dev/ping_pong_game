use bevy::prelude::{Query, Sprite, Transform};
use bevy::sprite::collide_aabb::collide;
use rand::Rng;
use crate::entities::{Ball, Paddle1p, Paddle2p};

// function to bound the ball
pub fn bound_ball(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    paddle_1p_query: Query<(&Paddle1p, &Transform, &Sprite)>,
    paddle_2p_query: Query<(&Paddle2p, &Transform, &Sprite)>
) {
    for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
        let ((_, paddle_1p_transform, paddle_1p_sprite), (_, paddle_2p_transform, paddle_2p_sprite)) = {
            (paddle_1p_query.single().unwrap(), paddle_2p_query.single().unwrap())
        };
        // check if ball is collided with top or bottom of the window
        if ball_transform.translation.y <= -242.5 || ball_transform.translation.y >= 242.5 {
            // rotate the ball
            ball.degree = 180.0 - ball.degree;
        }

        // check if ball is collided with paddle
        if collide(
            ball_transform.translation,
            ball_sprite.size,
            paddle_1p_transform.translation,
            paddle_1p_sprite.size
        ).is_some() ||
            collide(
                ball_transform.translation,
                ball_sprite.size,
                paddle_2p_transform.translation,
                paddle_2p_sprite.size
            ).is_some() {
            // rotate the ball
            ball.degree = 360.0 - ball.degree;
            // rotate the ball from -10.0 to 10.0
            {
                let mut rng = rand::thread_rng();
                ball.degree += rng.gen_range(-10.0..=10.0);
            }
            // make ball's degree between 0 and 360
            if ball.degree < 0.0 {
                ball.degree = 360.0 + ball.degree;
            }
            // add ball's speed
            ball.speed += 0.2
        }
    }
}