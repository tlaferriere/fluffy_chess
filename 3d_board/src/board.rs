use crate::movement::{AttemptMove, Piece, PlayerTurn, Square};
use bevy::math::vec4;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct SelectedSquare {
    pub entity: Option<Entity>,
}

#[derive(Resource, Default, Debug)]
pub(crate) struct SelectedPiece {
    pub entity: Option<Entity>,
}

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.2, -0.2, 0.4, 0.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.3, -0.3, 0.5, 0.0),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.3, 0.2, -0.3, 0.0),
        ..matl.to_owned()
    })),
};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_systems(Startup, create_board);
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        subdivisions: 0,
    }));

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        materials.add(Color::rgb(1., 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0., 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
                Square { x: i, y: j },
                On::<Pointer<Select>>::run(select),
            ));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn select(
    select: Listener<Pointer<Select>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    mut attempt_moves: EventWriter<AttemptMove>,
    squares_query: Query<&Square>,
    pieces_query: Query<(Entity, &mut Piece)>,
) {
    // Get the square under the cursor and set it as the selected
    selected_square.entity = Some(select.target);
    let Ok(square) = squares_query.get(select.target) else {
        return;
    };
    if let Some(selected_piece_entity) = selected_piece.entity {
        attempt_moves.send(AttemptMove {
            piece: selected_piece_entity,
            square: select.target,
        });
        // Reset selection after attempting a move
        selected_square.entity = None;
        selected_piece.entity = None;
    } else {
        selected_piece.entity = pieces_query.iter().find_map(|(entity, piece)| {
            if piece.x == square.x && piece.y == square.y && piece.color == turn.color {
                Some(entity)
            } else {
                None
            }
        });
    }
}
