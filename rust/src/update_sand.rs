use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::resources::{Grid, SpawnTimer, ColorState};
use crate::constants::*;
use crate::resources::MousePressed;

pub fn update_sand(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut grid: ResMut<Grid>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut color_state: ResMut<ColorState>,
    pressed: ResMut<MousePressed>
) {
    let mut new_grid = vec![vec![0.0; GRID_WIDTH]; GRID_HEIGHT];
    let mut rng = rand::thread_rng();
    let direction = if rng.gen_bool(0.5) { 1 } else { -1 };

    // Spawn sand on tick
    if timer.0.tick(time.delta()).just_finished() {
        color_state.hue = (color_state.hue + 1.0) % 360.0;
        new_grid[0][GRID_WIDTH / 10] = color_state.hue;
        new_grid[0][GRID_WIDTH / 2] = color_state.hue;
    }

    // Spawn on click
    if pressed.state {
        if let Ok(q_window) = q_window.get_single() {
            if let Some(position) = q_window.cursor_position() {
                let grid_x = (position.x / SCALE as f32) as usize;
                let grid_y = (position.y / SCALE as f32) as usize;
                
                if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                    grid.cells[grid_y][grid_x] = color_state.hue;
                }
            }
        }
    }

    // Update sand
    for y in (0..GRID_HEIGHT).rev() {
        for x in (0..GRID_WIDTH).rev() {
            if grid.cells[y][x] > 0.0 { // skip if 0
                if y + 1 < GRID_HEIGHT { // y would be under grid
                    if grid.cells[y + 1][x] == 0.0 { // space under is free
                        new_grid[y + 1][x] = grid.cells[y][x];
                    } else if x + 1 < GRID_WIDTH && direction == 1 { // check right
                        if grid.cells[y + 1][x + 1] == 0.0 { // check right under
                            new_grid[y + 1][x + 1] = grid.cells[y][x];
                        } else {
                            new_grid[y][x] = grid.cells[y][x];
                        }
                    } else if x > 0 && direction == -1 { // check left
                        if grid.cells[y + 1][x - 1] == 0.0 { // check left under
                            new_grid[y + 1][x - 1] = grid.cells[y][x];
                        } else {
                            new_grid[y][x] = grid.cells[y][x];
                        }
                    } else {
                        new_grid[y][x] = grid.cells[y][x];
                    }
                } else {
                    new_grid[y][x] = grid.cells[y][x];
                }
            }
        }
    }

    grid.cells = new_grid.clone();
}