use bevy::prelude::*;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600,
            height: 1600,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
