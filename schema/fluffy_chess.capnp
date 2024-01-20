@0xb05644efd3704caa;

interface GameMaker {
    findGame @0 (game_config: GameConfig, player: Player) -> (game_side: GameSide);
    # Returns a side of a game that fits the config.

    resumeGame @1 (id: UInt64, player: Player) -> (game_side: GameSide);
    # Resume a saved game based on it's ID.
}

interface GameHistoryService {
    struct Pagination {
        union {
            unpaginated @0: Void;
            paginated: group {
                limit @1: UInt16;
                offset @2: UInt64;
            }
        }
    }

    games @0 (username: Text, pagination: Pagination) -> (games: List(Game));
    # List of games
}

interface GameSide {
    # One side of the game. Both players will get a reference to one such side of the game.

    id @0 () -> (id: UInt64);
    # Return a unique ID to resume an unfinished game in case this connection is lost.

    color @1 () -> (color: Color);
    # Get the current side color.

    move @2 (move: Move);
    # Make a move when it is your turn. Saves the move.
}

interface Player {
    move @0 (move: Move);
    # Notify the player when a move has been made.
    # Should only return if the move has been processed and the player is ready make a move.
}

struct Game {
    moves @0: List(Move);
    ended @1: Bool;
}


struct GameConfig {
    user @0: Text;
    adversary: union {
        any @1: Void;
        friends @2: Void;
        user @3: Text;
    }
    color: union {
        any @4: Void;
        color @5: Color;
    }
    timer: union {
        # Seconds
        none @6: Void;
        perTurn @7: UInt32;
        perGame @8: UInt32;
    }
}

enum Color {
    white @0;
    black @1;
}

struct Move {
    piece @0: Piece;
    square @1: Square;
}

struct Piece {
    color @0: Color;
    type @1: Type;
    square @2: Square;

    enum Type {
        king @0;
        queen @1;
        bishop @2;
        knight @3;
        rook @4;
        pawn @5;
    }
}

struct Square {
    x @0: UInt8;
    y @1: UInt8;
}
