use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::WindowResolution};

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::linear_rgb(0.0, 0.0, 0.0)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(400.0, 400.0),
            title: "Falling Sand".to_string(),
            ..default()
        }),
        ..default()
    }));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>
) {
    commands.spawn(Camera2dBundle::default());

    // Create and spawn the red sand
    let scale: f32 = 5.0;
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let x_position = -window_width / 2.0 + scale / 2.0;
    let y_position = window_height / 2.0 - scale / 2.0;

    let sand = Mesh2dHandle(meshes.add(Rectangle::new(scale, scale)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: sand,
        material: materials.add(Color::linear_rgb(1.0, 0.0, 0.0)),
        transform: Transform::from_xyz(x_position, y_position, 0.0),
        ..default()
    });
}