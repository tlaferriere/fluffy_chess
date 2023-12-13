use crate::pieces::{Piece, PieceColor};
use bevy::math::vec4;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

#[derive(Resource, Default, Debug)]
pub struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Resource, Default, Debug)]
pub(crate) struct SelectedPiece {
    pub entity: Option<Entity>,
}

#[derive(Resource)]
pub(crate) struct PlayerTurn(pub(crate) PieceColor);
impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
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
            .init_resource::<PlayerTurn>()
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

fn select(
    mut commands: Commands,
    select: Listener<Pointer<Select>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    // Get the square under the cursor and set it as the selected
    selected_square.entity = Some(select.target);
    let Ok(square) = squares_query.get(select.target) else {
        return;
    };
    if let Some(selected_piece_entity) = selected_piece.entity {
        move_to_square(
            commands,
            &mut selected_square,
            &mut selected_piece,
            &mut turn,
            &mut pieces_query,
            square,
            selected_piece_entity,
        );
    } else {
        selected_piece.entity = pieces_query.iter().find_map(|(entity, piece)| {
            if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
                Some(entity)
            } else {
                None
            }
        });
    }
}

pub fn move_to_square(
    mut commands: Commands,
    selected_square: &mut ResMut<SelectedSquare>,
    selected_piece: &mut ResMut<SelectedPiece>,
    turn: &mut ResMut<PlayerTurn>,
    pieces_query: &mut Query<(Entity, &mut Piece)>,
    square: &Square,
    selected_piece_entity: Entity,
) {
    let pieces_vec = pieces_query.iter().map(|(_, piece)| *piece).collect();
    let pieces_entity_vec: Vec<(Entity, Piece)> = pieces_query
        .iter()
        .map(|(entity, piece)| (entity, *piece))
        .collect();
    if let Ok((_, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
        if piece.is_move_valid((square.x, square.y), pieces_vec) {
            // Check if a piece of the opposite color exists in this square and despawn it
            if let Some((other_entity, other_piece)) =
                pieces_entity_vec
                    .iter()
                    .find(|(other_entity, other_piece)| {
                        other_piece.x == square.x
                            && other_piece.y == square.y
                            && other_piece.color != piece.color
                    })
            {
                commands.entity(*other_entity).despawn_recursive()
            }

            // Move piece
            piece.x = square.x;
            piece.y = square.y;

            turn.0 = match turn.0 {
                PieceColor::White => PieceColor::Black,
                PieceColor::Black => PieceColor::White,
            }
        }
    }
    selected_square.entity = None;
    selected_piece.entity = None;
}
