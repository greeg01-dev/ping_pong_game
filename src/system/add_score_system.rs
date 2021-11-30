use bevy::prelude::{Query, Text, Transform, Vec3};
use rand::Rng;
use crate::entities::{Ball, Score};

// function to add score
pub fn add_score(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut score_query: Query<(&mut Score, &mut Text)>
) {
    for (mut ball, mut ball_transform) in ball_query.iter_mut() {
        // add score when ball went out of the window
        if ball_transform.translation.x > 492.5 {
            // reset ball's transform, speed, and degree
            ball_transform.translation = Vec3::new(0.0, 0.0, 1.0);
            ball.speed = 3.0;
            ball.degree = {
                let mut rng = rand::thread_rng();
                rng.gen_range(0.0..360.0) // make ball's degree from 0.0 to 360.0
            };

            // add player's score
            for (mut score, mut score_text) in score_query.iter_mut() {
                score.score_1p += 1;
                score_text.sections[0].value = format!("{}", score.score_1p)
            }
        }
        // add score when ball went out of the window
        if ball_transform.translation.x < -492.5 {
            // reset ball's transform, speed, and degree
            ball_transform.translation = Vec3::new(0.0, 0.0, 1.0);
            ball.speed = 3.0;
            ball.degree = {
                let mut rng = rand::thread_rng();
                rng.gen_range(0.0..360.0) // make ball's degree from 0.0 to 360.0
            };

            // add player's score
            for (mut score, mut score_text) in score_query.iter_mut() {
                score.score_2p += 1;
                score_text.sections[0].value = format!("{}", score.score_2p)
            }
        }
    }
}