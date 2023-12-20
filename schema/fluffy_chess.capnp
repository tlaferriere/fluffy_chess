@0xb05644efd3704caa;

struct Move {
    piece @0: Piece;
    square @1: Square;

    struct Piece {
        color @0: Color;
        type @1: Type;
        square @2: Square;

        enum Color {
            white @0;
            black @1;
        }

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
}