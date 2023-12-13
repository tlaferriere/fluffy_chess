mod pieces;

use pieces::*;

mod board;
use board::*;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa::default())
        // Set WindowDescriptor Resource to change title and size
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Chess!".to_string(),
                    resolution: WindowResolution::new(800., 800.),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            DefaultPickingPlugins,
            BoardPlugin,
            PiecesPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        });
    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}
