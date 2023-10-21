use crate::*;

use pretty_assertions::assert_eq;

macro_rules! make_piece {
    ( p ) => {
        Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::Black,
        })
    };
    ( n ) => {
        Some(Piece {
            kind: PieceKind::Knight,
            color: Color::Black,
        })
    };
    ( b ) => {
        Some(Piece {
            kind: PieceKind::Bishop,
            color: Color::Black,
        })
    };
    ( r ) => {
        Some(Piece {
            kind: PieceKind::Rook,
            color: Color::Black,
        })
    };
    ( q ) => {
        Some(Piece {
            kind: PieceKind::Queen,
            color: Color::Black,
        })
    };
    ( k ) => {
        Some(Piece {
            kind: PieceKind::King,
            color: Color::Black,
        })
    };

    ( P ) => {
        Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        })
    };
    ( N ) => {
        Some(Piece {
            kind: PieceKind::Knight,
            color: Color::White,
        })
    };
    ( B ) => {
        Some(Piece {
            kind: PieceKind::Bishop,
            color: Color::White,
        })
    };
    ( R ) => {
        Some(Piece {
            kind: PieceKind::Rook,
            color: Color::White,
        })
    };
    ( Q ) => {
        Some(Piece {
            kind: PieceKind::Queen,
            color: Color::White,
        })
    };
    ( K ) => {
        Some(Piece {
            kind: PieceKind::King,
            color: Color::White,
        })
    };
    ( - ) => {
        None
    };
}

macro_rules! make_rank {
    ( [$($piece:tt)+] ) => {
        [
            $(make_piece!($piece),)+
        ]
    }
}

macro_rules! make_board {
    ( $($rank:tt),+ $(,)? ) => {
        [
            $(make_rank!($rank),)+
        ]
    }
}

#[test]
fn starting_position() {
    assert_eq!(
        parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        Ok(Pfen {
            board: make_board![
                [ r n b q k b n r ],
                [ p p p p p p p p ],
                [ - - - - - - - - ],
                [ - - - - - - - - ],
                [ - - - - - - - - ],
                [ - - - - - - - - ],
                [ P P P P P P P P ],
                [ R N B Q K B N R ],
            ],

            turn: Color::White,

            castling: Castling {
                black_kingside: true,
                white_kingside: true,
                black_queenside: true,
                white_queenside: true,
            },

            en_passant: None,
            halfmove: 0,
            fullmove: 1,
        })
    );
}

#[test]
fn mid_game() {
    assert_eq!(
        parse("rnb1kb1r/pp1ppp1p/8/P1p3N1/4n3/8/P1PPQPPP/RNB1KB1R b KQkq - 0 6"),
        Ok(Pfen {
            board: make_board![
                [ r n b - k b - r ],
                [ p p - p p p - p ],
                [ - - - - - - - - ],
                [ P - p - - - N - ],
                [ - - - - n - - - ],
                [ - - - - - - - - ],
                [ P - P P Q P P P ],
                [ R N B - K B - R ],
            ],

            turn: Color::Black,

            castling: Castling {
                black_kingside: true,
                white_kingside: true,
                black_queenside: true,
                white_queenside: true,
            },

            en_passant: None,
            halfmove: 0,
            fullmove: 6,
        })
    );
}
