use crate::board::{move_to_square, SelectedPiece, SelectedSquare, Square};
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

fn select(
    listener: Listener<Pointer<Select>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    squares_query: Query<&Square>,
) {
    match selected_piece.entity {
        None => {
            selected_piece.entity = Some(listener.listener());
            println!("Selected piece {selected_piece:?}");
        }
        Some(selected_piece_entity) => {
            println!("Moving piece {selected_piece:?}");
            let Ok((_, piece)) = pieces_query.get(listener.listener()) else {
                return;
            };
            let Some(square) = squares_query
                .iter()
                .find(|square| piece.x == square.x && piece.y == square.y)
            else {
                return;
            };
            move_to_square(
                &mut selected_square,
                &mut selected_piece,
                &mut pieces_query,
                square,
                selected_piece_entity,
            );
            println!("Moved piece {selected_piece:?}");
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

#[derive(Component, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy, Component)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // Current position
    pub x: u8,
    pub y: u8,
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

/// Returns None if square is empty, returns a Some with the color if not
fn color_of_square(pos: (u8, u8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }
    None
}

fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Vec<Piece>) -> bool {
    // Same column
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    // Same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }

    // Diagonals
    let x_diff = (begin.0 as i8 - end.0 as i8).abs();
    let y_diff = (begin.1 as i8 - end.1 as i8).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                // left bottom - right top
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                // left top - right bottom
                (begin.0 + i as u8, begin.1 - i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                // right bottom - left top
                (begin.0 - i as u8, begin.1 + i as u8)
            } else {
                // begin.0 > end.0 && begin.1 > end.1
                // right top - left bottom
                (begin.0 - i as u8, begin.1 - i as u8)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}

impl Piece {
    /// Returns the possible_positions that are available
    pub fn is_move_valid(&self, new_position: (u8, u8), pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        let square_color = color_of_square(new_position, &pieces);
        if square_color == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                ((self.x as i8 - new_position.0 as i8).abs() == 1
                    && (self.y == new_position.1))
                    // Vertical
                    || ((self.y as i8 - new_position.1 as i8).abs() == 1
                    && (self.x == new_position.0))
                    // Diagonal
                    || ((self.x as i8 - new_position.0 as i8).abs() == 1
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
            }
            PieceType::Queen => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
                        || ((self.x == new_position.0 && self.y != new_position.1)
                            || (self.y == new_position.1 && self.x != new_position.0)))
            }
            PieceType::Bishop => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && (self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
            }
            PieceType::Knight => {
                ((self.x as i8 - new_position.0 as i8).abs() == 2
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
                    || ((self.x as i8 - new_position.0 as i8).abs() == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 2)
            }
            PieceType::Rook => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x == new_position.0 && self.y != new_position.1)
                        || (self.y == new_position.1 && self.x != new_position.0))
            }
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    // Normal move
                    new_position.0 as i8 - self.x as i8 == 1
                        && (self.y == new_position.1)
                        && square_color.is_none()
                    ||

                    // Move 2 squares
                     self.x == 1
                        && new_position.0 as i8 - self.x as i8 == 2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                        && square_color.is_none()
                    ||

                    // Take piece
                     new_position.0 as i8 - self.x as i8 == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                        && square_color == Some(PieceColor::Black)
                } else {
                    // Normal move
                    new_position.0 as i8 - self.x as i8 == -1
                        && (self.y == new_position.1)
                        && square_color.is_none()
                    ||

                    // Move 2 squares
                    self.x == 6
                        && new_position.0 as i8 - self.x as i8 == -2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                        && square_color.is_none()
                    ||

                    // Take piece
                    new_position.0 as i8 - self.x as i8 == -1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                        && square_color == Some(PieceColor::White)
                }
            }
        }
    }
}
