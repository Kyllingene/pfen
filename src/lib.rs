#![cfg_attr(not(test), no_std)]

use core::str::FromStr;

use thiserror_no_std::Error;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq, Error, Hash)]
pub enum PfenError<'fen> {
    #[error("too few rows: {0}")]
    TooFewRows(&'fen str),
    #[error("too many rows: {0}")]
    TooManyRows(&'fen str),
    #[error("row is too long: `{0}`")]
    RowTooLong(&'fen str),
    #[error("row is too short: `{0}`")]
    RowTooShort(&'fen str),
    #[error("invalid piece: {0}")]
    InvalidPiece(char),
    #[error("invalid color: `{0}`")]
    InvalidColor(&'fen str),
    #[error("invalid castling state: `{0}`")]
    InvalidCastling(&'fen str),
    #[error("invalid en passant state: `{0}`")]
    InvalidEnPassant(&'fen str),
    #[error("invalid halfmove: `{0}`")]
    InvalidHalfmove(&'fen str),
    #[error("invalid fullmove: `{0}`")]
    InvalidFullmove(&'fen str),

    #[error("too few segments in fen string; expected 6, found {0}")]
    TooFewSegments(usize),
    #[error("too many segments in fen string; expected 6, found {0}")]
    TooManySegments(usize),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl TryFrom<char> for Piece {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        Ok(match ch {
            'p' => Self {
                kind: PieceKind::Pawn,
                color: Color::Black,
            },
            'n' => Self {
                kind: PieceKind::Knight,
                color: Color::Black,
            },
            'b' => Self {
                kind: PieceKind::Bishop,
                color: Color::Black,
            },
            'r' => Self {
                kind: PieceKind::Rook,
                color: Color::Black,
            },
            'q' => Self {
                kind: PieceKind::Queen,
                color: Color::Black,
            },
            'k' => Self {
                kind: PieceKind::King,
                color: Color::Black,
            },

            'P' => Self {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            'N' => Self {
                kind: PieceKind::Knight,
                color: Color::White,
            },
            'B' => Self {
                kind: PieceKind::Bishop,
                color: Color::White,
            },
            'R' => Self {
                kind: PieceKind::Rook,
                color: Color::White,
            },
            'Q' => Self {
                kind: PieceKind::Queen,
                color: Color::White,
            },
            'K' => Self {
                kind: PieceKind::King,
                color: Color::White,
            },

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

impl core::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Castling {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl FromStr for Castling {
    type Err = ();

    fn from_str(castling: &str) -> Result<Self, Self::Err> {
        let mut out = Self::default();

        if castling == "-" {
            return Ok(out);
        } else if castling.len() > 4 || castling.is_empty() {
            return Err(());
        }

        for ch in castling.chars() {
            match ch {
                'q' => out.black_queenside = true,
                'Q' => out.white_queenside = true,
                'k' => out.black_kingside = true,
                'K' => out.white_kingside = true,

                _ => return Err(()),
            }
        }

        Ok(out)
    }
}

fn parse_en_passant(ep: &str) -> Option<Option<(u8, u8)>> {
    if ep == "-" {
        return Some(None);
    } else if ep.len() != 4 {
        return None;
    }

    let mut chars = ep.chars();
    let file = chars.next().unwrap();
    let rank = chars.next().unwrap();

    let file = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,

        _ => return None,
    };

    let rank = match rank {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,

        _ => return None,
    };

    Some(Some((file, rank)))
}

/// A chessboard as parsed by pfen.
///
/// Ordered from bottom-right (white kingside rook) to top-left (black queenside rook),
/// in row-major format.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pfen {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    pub castling: Castling,
    pub en_passant: Option<(u8, u8)>,
    pub halfmove: u32,
    pub fullmove: u32,
}

pub fn parse(fen: &str) -> Result<Pfen, PfenError> {
    let mut board = Pfen::default();

    let mut sections = fen.split(' ');
    let too_few = PfenError::TooFewSegments;

    let pieces = sections.next().ok_or(too_few(0))?;

    let turn = sections.next().ok_or(too_few(1))?;
    if turn.len() == 1 {
        board.turn = turn
            .chars()
            .next()
            .unwrap()
            .try_into()
            .map_err(|_| PfenError::InvalidColor(turn))?;
    } else {
        return Err(PfenError::InvalidColor(turn));
    }

    let castling = sections.next().ok_or(too_few(2))?;
    board.castling = Castling::from_str(castling)
        .map_err(|_| PfenError::InvalidCastling(castling))?;

    let en_passant = sections.next().ok_or(too_few(3))?;
    board.en_passant = parse_en_passant(en_passant)
        .ok_or(PfenError::InvalidEnPassant(en_passant))?;

    let halfmove = sections.next().ok_or(too_few(4))?;
    board.halfmove = halfmove
        .parse()
        .map_err(|_| PfenError::InvalidHalfmove(halfmove))?;

    let fullmove = sections.next().ok_or(too_few(5))?;
    board.fullmove = fullmove
        .parse()
        .map_err(|_| PfenError::InvalidFullmove(fullmove))?;

    let mut ri = 0;
    for row in pieces.split('/') {
        if ri > 8 {
            return Err(PfenError::TooManyRows(pieces));
        }

        let mut len = 0;
        for ch in row.chars() {
            if ch.is_numeric() {
                let num = match ch {
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,

                    _ => return Err(PfenError::RowTooLong(row)),
                };

                len += num;
            } else {
                let piece = ch.try_into().map_err(|_| PfenError::InvalidPiece(ch))?;
                board.board[7 - ri][len] = Some(piece);

                len += 1;
            }

            if len > 8 {
                return Err(PfenError::RowTooLong(row));
            }
        }

        if len < 8 {
            return Err(PfenError::RowTooShort(row));
        }

        ri += 1;
    }

    if ri < 7 {
        return Err(PfenError::TooFewRows(pieces));
    }

    if sections.next().is_some() {
        Err(PfenError::TooManySegments(9 + sections.count()))
    } else {
        Ok(board)
    }
}
