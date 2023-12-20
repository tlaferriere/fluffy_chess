use crate::movement::{PieceColor, PlayerTurn};
use bevy::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_next_move_text)
            .add_systems(Update, next_move_text_update);
    }
}

// Component to mark the Text entity
#[derive(Component)]
struct NextMoveText;

/// Initialize UiCamera and text
fn init_next_move_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Next move: White".to_string(),
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                },
                NextMoveText,
            ));
        });
}

/// Update text with the correct turn
fn next_move_text_update(turn: Res<PlayerTurn>, mut query: Query<(&mut Text, &NextMoveText)>) {
    if !turn.is_changed() {
        return;
    }
    let Ok((mut text, _tag)) = query.get_single_mut() else {
        return;
    };
    let Some(section) = text.sections.get_mut(0) else {
        return;
    };
    section.value = format!(
        "Next move: {}",
        match turn.color {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
        }
    );
}
