use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::render_asset::RenderAssetUsages;
use crate::constants::*;
use crate::resources::SandDisplay;

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>
) {
    commands.spawn(Camera2dBundle::default());

    // Create initial texture
    let image = Image::new_fill(
        Extent3d {
            width: WIN_X,
            height: WIN_Y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &vec![0; (WIN_X * WIN_Y * 4) as usize],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    );

    // Spawn sprite with texture
    commands.spawn((
        SpriteBundle {
            texture: images.add(image),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIN_X as f32, WIN_Y as f32)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        SandDisplay
    ));
}