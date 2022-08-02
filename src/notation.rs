use std::{error::Error, fmt::Display, num::ParseIntError};

use regex::Regex;

/// Struct that represents Universal Notation Data,
/// used for describing positions of any fairy board game
pub(crate) struct UniversalNotation {}
impl From<Fen> for UniversalNotation {
    fn from(fen: Fen) -> Self {
        todo!()
    }
}
impl From<Sfen> for UniversalNotation {
    fn from(sfen: Sfen) -> Self {
        todo!()
    }
}

/// Struct that represents [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) (FEN) data,
/// used for describing positions of Chess games
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Fen {
    pub(crate) piece_data: String,
    pub(crate) active_color: String,
    pub(crate) castling: Option<String>,
    pub(crate) en_passant_target: Option<String>,
    pub(crate) halfmove_clock: i32,
    pub(crate) move_number: i32,
}
impl Fen {
    /// Contructs a [`Fen`] object from raw FEN string
    ///
    /// Example of a valid FEN notation string: `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1` (The starting position)
    ///
    /// # Errors
    ///
    /// This function will return [`NotationError::InvalidNotation`] if provided string doesn't match standard notation
    pub(crate) fn from_raw(input: &str) -> Result<Fen, NotationError> {
        let mut data = input.split_whitespace();

        if data.clone().count() != 6 {
            return Err(NotationError::InvalidNotation);
        };

        let piece_data = Fen::parse_notation(r"(([rnbqkpRNBQKP]|[1-9]|[/]){1,9}){8}", data.next().unwrap())?;
        let active_color = Fen::parse_notation(r"[bw]", data.next().unwrap())?;
        let castling = Fen::parse_notation(r"([KQkq]{1,4}|-){1}", data.next().unwrap())?;
        let target = Fen::parse_notation(r"([a-h][1-8]|-){1}", data.next().unwrap())?;
        let halfmove_clock = data.next().unwrap().parse()?;
        let move_number = data.next().unwrap().parse()?;

        Ok(Fen {
            piece_data,
            active_color,
            castling:          if castling == "-"  { None } else { Some(castling) },
            en_passant_target: if   target == "-"  { None } else { Some(target) },
            halfmove_clock,
            move_number,
        })
    }

    fn parse_notation(re: &str, raw: &str) -> Result<String, NotationError> {
        let re = Regex::new(re).unwrap();
        if !re.is_match(raw) {
            return Err(NotationError::InvalidNotation);
        }

        Ok(re
            .captures(raw)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_owned())
    }
}

/// SFEN is an extension of [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) [`Fen`]
/// used for describing board positions of shogi games.
pub(crate) struct Sfen {
    piece_data: String,
    active_color: String,
    hand_piece_data: String,
}
impl Sfen {
    /// Contructs a [`Sfen`] notation object from raw (Western) SFEN string
    ///
    /// Example of a valid SFEN notation string: `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`
    ///
    /// # Errors
    ///
    /// This function will return [`NotationError::InvalidNotation`] if provided string doesn't match the standard notation
    pub(crate) fn from_raw(input: &str) -> Result<Sfen, NotationError> {
        let mut data = input.split_whitespace();

        if data.clone().count() != 3 {
            return Err(NotationError::InvalidNotation);
        };

        let piece_data = data.next().unwrap().to_owned();
        if !Regex::new(r"(([plnsgkPLNSGK]|[1-9]|[/]|[+]){1,9}){9}")
            .unwrap()
            .is_match(&piece_data)
        {
            return Err(NotationError::InvalidNotation);
        };

        let active_color = data.next().unwrap().to_owned();
        if !Regex::new(r"[wb]").unwrap().is_match(&active_color) {
            return Err(NotationError::InvalidNotation);
        };

        let hand_piece_data = data.next().unwrap().to_owned();
        if !Regex::new(r"(([plnsgkPLNSGK]|[1-9]|[/]|[+]){1,9}){9}")
            .unwrap()
            .is_match(&hand_piece_data)
        {
            return Err(NotationError::InvalidNotation);
        };

        Ok(Sfen {
            piece_data,
            active_color,
            hand_piece_data,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NotationError {
    InvalidNotation,
    InvalidPiece(String),
}
impl Display for NotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotationError::InvalidNotation => write!(f, "Notation is invalid"),
            NotationError::InvalidPiece(piece) => write!(f, "\"{}\" piece is invalid", piece),
        }
    }
}
impl Error for NotationError {}
impl From<ParseIntError> for NotationError {
    fn from(_: ParseIntError) -> Self {
        NotationError::InvalidNotation
    }
}

#[cfg(test)]
mod tests {
    use crate::notation::NotationError;

    #[test]
    fn standard_fen() {
        let fen = super::Fen::from_raw("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(fen.piece_data, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(fen.active_color, "w");
        assert_eq!(fen.castling, Some("KQkq".to_owned()));
        assert_eq!(fen.en_passant_target, None);
        assert_eq!(fen.halfmove_clock, 0);
        assert_eq!(fen.move_number, 1);
    }

    #[test]
    fn no_castling() {
        let fen = super::Fen::from_raw("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1").unwrap();
        assert_eq!(fen.castling, None);
        assert_eq!(fen.en_passant_target, None);
    }

    #[test]
    fn empty_board() {
        let fen = super::Fen::from_raw("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(fen.piece_data, "8/8/8/8/8/8/8/8");
    }

    #[test]
    fn max_move_number() {
        let fen = super::Fen::from_raw("8/8/8/8/8/8/8/8 w - - 2147483647 2147483647").unwrap();
        assert_eq!(fen.halfmove_clock, i32::MAX);
        assert_eq!(fen.move_number, i32::MAX);
    }

    #[test]
    fn invalid_move_number() {
        let fen = super::Fen::from_raw("8/8/8/8/8/8/8/8 w - - 9999999999 9999999999").unwrap_err();
        assert_eq!(fen, NotationError::InvalidNotation);
    }

    #[test]
    fn invalid_active_color() {
        let fen = super::Fen::from_raw("8/8/8/8/8/8/8/8 no - - 0 1").unwrap_err();
        assert_eq!(fen, NotationError::InvalidNotation);
    }

    #[test]
    fn invalid_castling_rights() {
        let fen = super::Fen::from_raw("8/8/8/8/8/8/8/8 w SoBad - 0 1").unwrap_err();
        assert_eq!(fen, NotationError::InvalidNotation);
    }
}
