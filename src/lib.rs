use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error, Hash)]
pub enum PfenError {
    #[error("invalid piece: {0}")]
    InvalidPiece(char),
    #[error("invalid color: `{0}`")]
    InvalidColor(String),
    #[error("invalid halfmove: `{0}`")]
    InvalidHalfmove(String),
    #[error("invalid fullmove: `{0}`")]
    InvalidFullmove(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Piece {
    kind: PieceKind,
    color: Color,
}

impl TryFrom<char> for Piece {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        Ok(match ch {
            'p' => Self { kind: PieceKind::Pawn, color: Color::Black },
            'n' => Self { kind: PieceKind::Knight, color: Color::Black },
            'b' => Self { kind: PieceKind::Bishop, color: Color::Black },
            'r' => Self { kind: PieceKind::Rook, color: Color::Black },
            'q' => Self { kind: PieceKind::Queen, color: Color::Black },
            'k' => Self { kind: PieceKind::King, color: Color::Black },

            'P' => Self { kind: PieceKind::Pawn, color: Color::White },
            'N' => Self { kind: PieceKind::Knight, color: Color::White },
            'B' => Self { kind: PieceKind::Bishop, color: Color::White },
            'R' => Self { kind: PieceKind::Rook, color: Color::White },
            'Q' => Self { kind: PieceKind::Queen, color: Color::White },
            'K' => Self { kind: PieceKind::King, color: Color::White },

            _ => Err(())?,
        })
    }
}

/// The kind of piece in a square.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PieceKind {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// A valid chess color, either white or black.
///
/// Defaults to white. `White > Black`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Black,
    #[default]
    White,
}

impl TryFrom<char> for Color {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        Ok(match ch {
            'w' => Self::White,
            'b' => Self::Black,

            _ => Err(())?,
        })
    }
}

/// The castling situation in a fen board.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Castling {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl FromStr for Castling {
    type Err = ();

    fn from_str(castling: &str) -> Result<Self, Self::Err> {
        if castling == "-" {
            return Self::default();
        }

        todo!()
    }
}

/// A chessboard as parsed by pfen.
///
/// Ordered from bottom-right (white kingside rook) to top-left (black queenside rook).
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Pfen {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    pub castling: (bool, bool),
    pub en_passant: Option<(u8, u8)>,
    pub halfmove: u32,
    pub fullmove: u32,
}

pub fn parse(fen: &str) -> Result<Pfen, PfenError> {
    let mut board = Pfen::default();
    
    let sections = fen.split(" ");

    Ok(board)
}

