mod pieces;
use pieces::*;

mod board;
use board::*;

mod movement;
use movement::*;

use bevy::prelude::*;
use bevy::time::Stopwatch;
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
            MovementPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component, Default)]
struct MainCamera {
    stopwatch: Stopwatch,
}

const CAMERA_ROT_WHITE: (Quat, Quat) = (
    Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5),
    Quat::from_xyzw(0.3, 0.5, 0.3, -0.5),
);
const CAMERA_ROT_BLACK: (Quat, Quat) = (
    Quat::from_xyzw(-0.3, 0.5, 0.3, 0.5),
    Quat::from_xyzw(0.3, -0.5, -0.3, -0.5),
);

fn setup(mut commands: Commands) {
    commands
        // Camera
        .spawn((
            Camera3dBundle {
                transform: Transform::from_matrix(Mat4::from_rotation_translation(
                    CAMERA_ROT_WHITE.0.normalize(),
                    Vec3::new(-7.0, 20.0, 4.0),
                )),
                ..Default::default()
            },
            MainCamera::default(),
        ));
    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn move_camera(
    time: Res<Time>,
    turn: Res<PlayerTurn>,
    mut moves: EventReader<movement::Move>,
    mut query: Query<(&mut Transform, &mut MainCamera)>,
) {
    let Ok((mut transform, mut camera)) = query.get_single_mut() else {
        return;
    };
    if let Some(_) = moves.read().next() {
        camera.stopwatch.reset();
    } else {
        camera.stopwatch.tick(time.delta());
    }
    // Get the direction to turn in
    let target = match turn.color {
        PieceColor::White => CAMERA_ROT_WHITE,
        PieceColor::Black => CAMERA_ROT_BLACK,
    };

    if (target.0.normalize() - transform.rotation.normalize()).length() > 0.01
        && (target.1.normalize() - transform.rotation.normalize()).length() > 0.01
    {
        transform.rotate_around(
            Vec3 {
                x: 4.,
                y: 0.,
                z: 4.,
            },
            Quat::from_rotation_y(180.0f32.to_radians() * time.delta_seconds()),
        );
    }
}
