use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    pub cells: Vec<Vec<f32>> // Using f32 for hue values (0-360)
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: vec![vec![0.0; super::constants::GRID_WIDTH]; super::constants::GRID_HEIGHT]
        }
    }
}

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Resource)]
pub struct ColorState {
    pub hue: f32
}

#[derive(Component)]
pub struct SandDisplay;