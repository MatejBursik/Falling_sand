mod constants;
mod resources;
mod setup;
mod update_sand;
mod update_texture;
mod functions;

use bevy::prelude::*;
use constants::*;
use resources::*;
use setup::setup;
use update_sand::update_sand;
use update_texture::update_texture;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WIN_X as f32, WIN_Y as f32).into(),
            title: "Falling Sand".to_string(),
            ..default()
        }),
        ..default()
    }));
    app.insert_resource(Grid::default());
    app.insert_resource(SpawnTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
    app.insert_resource(ColorState { hue: 0.0 });
    app.add_systems(Startup, setup);
    app.add_systems(Update, update_sand);
    app.add_systems(Update, update_texture);
    app.run();
}