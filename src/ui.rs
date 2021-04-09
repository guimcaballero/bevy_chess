use crate::{board::*, pieces::*};
use bevy::prelude::*;

// Component to mark the Text entity
struct NextMoveText;

/// Initialize UiCamera and text
fn init_next_move_text(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        .spawn_bundle(UiCameraBundle::default())
        // root node
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Next move: White",
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(NextMoveText);
        });
}

/// Update text with the correct turn
fn next_move_text_update(turn: Res<PlayerTurn>, mut query: Query<(&mut Text, &NextMoveText)>) {
    if !turn.is_changed() {
        return;
    }
    for (mut text, _tag) in query.iter_mut() {
        text.sections[0].value = format!(
            "Next move: {}",
            match turn.0 {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            }
        );
    }
}

/// Demo system to show off Query transformers
fn log_text_changes(query: Query<&Text, Changed<Text>>) {
    for text in query.iter() {
        println!("New text: {}", text.sections[0].value);
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_next_move_text.system())
            .add_system(next_move_text_update.system())
            .add_system(log_text_changes.system());
    }
}
