use bevy::prelude::{Assets, AssetServer, ColorMaterial, Commands, Handle, OrthographicCameraBundle, Res, ResMut, SpriteBundle, Texture, Transform, Vec3};
use rand::prelude::*;
use crate::entities::{Ball, Paddle1p, Paddle2p};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let paddle_material: Handle<Texture> = asset_server.load("image/paddle.png");
    let ball_material: Handle<Texture> = asset_server.load("image/ball.png");

    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // spawn paddle for 1p
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(paddle_material.clone().into()),
            transform: Transform {
                translation: Vec3::new(-420.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle1p);
    // spawn paddle for 2p
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(paddle_material.clone().into()),
            transform: Transform {
                translation: Vec3::new(420.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle2p);
    // spawn ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(ball_material.clone().into()),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            degree: {
                let mut rng = rand::thread_rng();
                rng.gen_range(0.0..360.0)
            },
            speed: 3.0
        });
}