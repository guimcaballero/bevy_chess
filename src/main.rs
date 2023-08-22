use bevy::{prelude::*, window::WindowResolution};
use bevy_mod_picking::prelude::*;

mod pieces;
use pieces::*;
mod board;
use board::*;
mod ui;
use ui::*;

fn main() {
    App::default()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa::Sample4)
        // Set WindowDescriptor Resource to change title and size
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess!".to_string(),
                resolution: WindowResolution::new(600., 600.),
                ..Default::default()
              }),
            ..default()
          }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(BoardPlugin)
        .add_plugins(PiecesPlugin)
        .add_plugins(UIPlugin)
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
        })
        .insert(RaycastPickCamera::default())   // Enable picking with this camera
        // Light
        .commands()
        .spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
