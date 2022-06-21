use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
use pieces::*;
mod board;
use board::*;
mod ui;
use ui::*;

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 600.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        // Camera
        .spawn_bundle(PerspectiveCameraBundle{
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                  Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                  Vec3::new(-7.0, 20.0, 4.0))),
            ..Default::default()

        })
        .insert_bundle(PickingCameraBundle::default())
        // Light
        .commands()
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
