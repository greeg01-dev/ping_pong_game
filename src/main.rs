#![windows_subsystem = "windows"]

use bevy::prelude::{App, ClearColor, Color, DefaultPlugins, WindowDescriptor};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("Ping Pong Game"),
            width: 1000.0,
            height: 500.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .run()
}