use bevy::prelude::*;
use bevy::render::texture::Image;
use crate::resources::{Grid, SandDisplay};
use crate::constants::*;
use crate::functions::hsv_to_rgb;

pub fn update_texture(
    mut images: ResMut<Assets<Image>>,
    sand_display: Query<&Handle<Image>, With<SandDisplay>>,
    grid: Res<Grid>
) {
    if let Ok(texture_handle) = sand_display.get_single() {
        if let Some(texture) = images.get_mut(texture_handle) {
            for y in 0..GRID_HEIGHT {
                for x in 0..GRID_WIDTH {
                    let hue = grid.cells[y][x];
                    let (r, g, b) = if hue > 0.0 {
                        hsv_to_rgb(hue, 1.0, 0.5)
                    } else {
                        (0.0, 0.0, 0.0)
                    };

                    for dy in 0..SCALE {
                        for dx in 0..SCALE {
                            let pixel_x = x as u32 * SCALE + dx;
                            let pixel_y = y as u32 * SCALE + dy;
                            let pixel_idx = ((pixel_y * WIN_X + pixel_x) * 4) as usize;
                            
                            texture.data[pixel_idx] = (r * 255.0) as u8;
                            texture.data[pixel_idx + 1] = (g * 255.0) as u8;
                            texture.data[pixel_idx + 2] = (b * 255.0) as u8;
                            texture.data[pixel_idx + 3] = 255;
                        }
                    }
                }
            }
        }
    }
}