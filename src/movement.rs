use bevy::app::AppExit;
use bevy::prelude::*;

pub struct GameTimer {
    pub reset: bool,
    pub white_time_left: Timer,
    pub black_time_left: Timer,
}

#[derive(Resource)]
pub struct PlayerTurn {
    pub color: PieceColor,
    timer: Option<GameTimer>,
}
impl Default for PlayerTurn {
    fn default() -> Self {
        Self {
            color: PieceColor::White,
            timer: None,
        }
    }
}

#[derive(Event)]
pub struct AttemptMove {
    pub piece: Entity,
    pub square: Entity,
}

#[derive(Event, Clone, Copy)]
pub struct Move {
    pub piece: Piece,
    pub square: Square,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttemptMove>()
            .add_event::<Move>()
            .init_resource::<PlayerTurn>()
            .add_systems(Update, (move_to_square, crate::move_camera));
    }
}

#[allow(clippy::too_many_arguments)]
pub fn move_to_square(
    mut commands: Commands,
    mut turn: ResMut<PlayerTurn>,
    mut attempted_moves: EventReader<AttemptMove>,
    mut moves: EventWriter<Move>,
    mut exit: EventWriter<AppExit>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    squares_query: Query<&Square>,
) {
    for attempted_move in attempted_moves.read() {
        let Ok(square) = squares_query.get(attempted_move.square) else {
            continue;
        };
        let pieces_vec = pieces_query.iter().map(|(_, piece)| *piece).collect();
        let pieces_entity_vec: Vec<(Entity, Piece)> = pieces_query
            .iter()
            .map(|(entity, piece)| (entity, *piece))
            .collect();
        let Ok((_, mut piece)) = pieces_query.get_mut(attempted_move.piece) else {
            continue;
        };
        if !(piece.is_move_valid((square.x, square.y), pieces_vec)) {
            continue;
        }
        // Check if a piece of the opposite color exists in this square and despawn it
        if let Some((other_entity, other_piece)) =
            pieces_entity_vec
                .iter()
                .find(|(_other_entity, other_piece)| {
                    other_piece.x == square.x
                        && other_piece.y == square.y
                        && other_piece.color != piece.color
                })
        {
            // If the king is taken, we should exit
            if other_piece.piece_type == PieceType::King {
                println!(
                    "{} won! Thanks for playing!",
                    match turn.color {
                        PieceColor::White => "White",
                        PieceColor::Black => "Black",
                    }
                );
                exit.send(AppExit);
            }
            commands.entity(*other_entity).despawn_recursive()
        }

        // Move piece
        let done_move = Move {
            piece: *piece,
            square: *square,
        };

        piece.x = square.x;
        piece.y = square.y;

        turn.color = match turn.color {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };

        // We need the information on the origin position of the piece
        assert!(done_move.square.x != done_move.piece.x || done_move.square.y != done_move.piece.y);

        moves.send(done_move);
    }
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

#[derive(Component, Clone, Copy)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}
