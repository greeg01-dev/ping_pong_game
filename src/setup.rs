use bevy::prelude::{Assets, AssetServer, Color, ColorMaterial, Commands, Handle, HorizontalAlign, OrthographicCameraBundle, Res, ResMut, SpriteBundle, Text, Text2dBundle, TextSection, TextStyle, Texture, Transform, UiCameraBundle, Vec3, VerticalAlign};
use bevy::text::TextAlignment;
use rand::Rng;
use crate::entities::{Ball, Paddle1p, Paddle2p, Score};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let paddle_material: Handle<Texture> = asset_server.load("image/paddle.png");
    let ball_material: Handle<Texture> = asset_server.load("image/ball.png");

    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    
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
                rng.gen_range(0.0..360.0) // make ball's degree from 0.0 to 360.0
            },
            speed: 3.0
        });
    // spawn score board
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: String::from("0"),
                        style: TextStyle {
                            font: asset_server.load("font/square.ttf"),
                            font_size: 100.0,
                            color: Color::BLACK
                        }
                    },
                    TextSection {
                        value: String::from(" : "),
                        style: TextStyle {
                            font: asset_server.load("font/square.ttf"),
                            font_size: 100.0,
                            color: Color::BLACK
                        }
                    },
                    TextSection {
                        value: String::from("0"),
                        style: TextStyle {
                            font: asset_server.load("font/square.ttf"),
                            font_size: 100.0,
                            color: Color::BLACK
                        }
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                }
            },
            transform: Transform {
                translation: Vec3::new(0.0, 170.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score { score_1p: 0, score_2p: 0 });
}