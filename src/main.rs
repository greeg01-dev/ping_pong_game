#![windows_subsystem = "windows"]

use bevy::prelude::{App, ClearColor, Color, DefaultPlugins, IntoSystem, WindowDescriptor};

mod setup;
mod entities;
mod system;

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
        .add_startup_system(setup::setup.system())
        .add_system(system::move_ball.system())
        .add_system(system::bound_ball.system())
        .add_system(system::move_paddle_1p.system())
        .add_system(system::move_paddle_2p.system())
        .add_system(system::score_board.system())
        .add_system(system::add_score.system())
        .run()

}