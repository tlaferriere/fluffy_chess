use crate::board::{SelectedPiece, SelectedSquare};
use crate::movement::{AttemptMove, Piece, PieceColor, PieceType, PlayerTurn, Square};
use bevy::math::vec4;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_pieces)
            .add_systems(Update, move_pieces);
    }
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    // Add some materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    let pieces_parent = commands.spawn((PbrBundle::default(),)).id();

    let white_pieces = [
        spawn_rook(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            rook_handle.clone(),
            (0, 0),
        ),
        spawn_knight(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            knight_1_handle.clone(),
            knight_2_handle.clone(),
            (0, 1),
        ),
        spawn_bishop(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            bishop_handle.clone(),
            (0, 2),
        ),
        spawn_queen(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            queen_handle.clone(),
            (0, 3),
        ),
        spawn_king(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            king_handle.clone(),
            king_cross_handle.clone(),
            (0, 4),
        ),
        spawn_bishop(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            bishop_handle.clone(),
            (0, 5),
        ),
        spawn_knight(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            knight_1_handle.clone(),
            knight_2_handle.clone(),
            (0, 6),
        ),
        spawn_rook(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            rook_handle.clone(),
            (0, 7),
        ),
    ];
    commands.entity(pieces_parent).push_children(&white_pieces);

    let mut white_pawns = [pieces_parent; 8];

    for i in 0..8u8 {
        white_pawns[i as usize] = spawn_pawn(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            pawn_handle.clone(),
            (1, i),
        );
    }
    commands.entity(pieces_parent).push_children(&white_pawns);

    let black_pieces = [
        spawn_rook(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            rook_handle.clone(),
            (7, 0),
        ),
        spawn_knight(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            knight_1_handle.clone(),
            knight_2_handle.clone(),
            (7, 1),
        ),
        spawn_bishop(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            bishop_handle.clone(),
            (7, 2),
        ),
        spawn_queen(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            queen_handle.clone(),
            (7, 3),
        ),
        spawn_king(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            king_handle.clone(),
            king_cross_handle.clone(),
            (7, 4),
        ),
        spawn_bishop(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            bishop_handle.clone(),
            (7, 5),
        ),
        spawn_knight(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            knight_1_handle.clone(),
            knight_2_handle.clone(),
            (7, 6),
        ),
        spawn_rook(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            rook_handle.clone(),
            (7, 7),
        ),
    ];

    commands.entity(pieces_parent).push_children(&black_pieces);

    let mut black_pawns = [pieces_parent; 8];
    for i in 0..8u8 {
        black_pawns[i as usize] = spawn_pawn(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            pawn_handle.clone(),
            (6, i),
        );
    }
    commands.entity(pieces_parent).push_children(&black_pawns);
}

#[allow(clippy::too_many_arguments)]
fn select(
    listener: Listener<Pointer<Select>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    mut attempt_move: EventWriter<AttemptMove>,
    pieces_query: Query<(Entity, &mut Piece)>,
    squares_query: Query<(Entity, &Square)>,
) {
    match selected_piece.entity {
        None => {
            if pieces_query
                .get(listener.listener())
                .is_ok_and(|(_, piece)| piece.color == turn.0)
            {
                selected_piece.entity = Some(listener.listener());
            }
        }
        Some(selected_piece_entity) => {
            let Ok((_, piece)) = pieces_query.get(listener.listener()) else {
                return;
            };
            let Some(square) = squares_query.iter().find_map(|(entity, square)| {
                if piece.x == square.x && piece.y == square.y {
                    Some(entity)
                } else {
                    None
                }
            }) else {
                return;
            };
            attempt_move.send(AttemptMove {
                piece: selected_piece_entity,
                square,
            });
            selected_piece.entity = None;
            selected_square.entity = None;
        }
    }
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: (u8, u8),
) -> Entity {
    commands
        // Spawn parent entity
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            On::<Pointer<Select>>::run(select),
            Piece {
                color: piece_color,
                piece_type: PieceType::King,
                x: position.0,
                y: position.1,
            },
        ))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
                        transform
                    },
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
            parent.spawn((
                PbrBundle {
                    mesh: mesh_cross,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
                        transform
                    },
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
        })
        .id()
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: (u8, u8),
) -> Entity {
    commands
        // Spawn parent entity
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            On::<Pointer<Select>>::run(select),
            Piece {
                color: piece_color,
                piece_type: PieceType::Knight,
                x: position.0,
                y: position.1,
            },
        ))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: mesh_1,
                    material: material.clone(),
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
                        transform
                    },
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
            parent.spawn((
                PbrBundle {
                    mesh: mesh_2,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
                        transform
                    },
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
        })
        .id()
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Entity {
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            On::<Pointer<Select>>::run(select),
            Piece {
                color: piece_color,
                piece_type: PieceType::Queen,
                x: position.0,
                y: position.1,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
                        transform
                    },
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
        })
        .id()
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Entity {
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            On::<Pointer<Select>>::run(select),
            Piece {
                color: piece_color,
                piece_type: PieceType::Bishop,
                x: position.0,
                y: position.1,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
                        transform
                    },
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
        })
        .id()
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Entity {
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            On::<Pointer<Select>>::run(select),
            Piece {
                color: piece_color,
                piece_type: PieceType::Rook,
                x: position.0,
                y: position.1,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
                        transform
                    },
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
        })
        .id()
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Entity {
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            On::<Pointer<Select>>::run(select),
            Piece {
                color: piece_color,
                piece_type: PieceType::Pawn,
                x: position.0,
                y: position.1,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                        transform.scale *= Vec3::new(0.2, 0.2, 0.2);
                        transform
                    },
                    ..Default::default()
                },
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
        })
        .id()
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * 8. * time.delta_seconds();
        }
    }
}
