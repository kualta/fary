use std::{error::Error, fmt::Display, num::ParseIntError};

use regex::Regex;

pub(crate) struct Fen {
    piece_data: String,
    active_color: String,
    castling: String,
    en_passant_target: Option<String>,
    halfmove_clock: i32,
    move_number: i32,
}
impl Fen {
    pub(crate) fn parse(input: &str) -> Result<Fen, FenError> {
        let mut data = input.split_whitespace();

        if data.clone().count() != 6 {
            return Err(FenError::InvalidNotation);
        };

        let piece_data = data.next().unwrap().to_owned();
        if !Regex::new(r"(([rnbqkpRNBQKP]|[1-9]|[/]){1,9}){8}").unwrap().is_match(&piece_data) {
            return Err(FenError::InvalidNotation);
        };

        let active_color = data.next().unwrap().to_owned();
        if !Regex::new(r"[wb]|-").unwrap().is_match(&active_color) {
            return Err(FenError::InvalidNotation);
        };

        let castling = data.next().unwrap().to_owned();
        if !Regex::new(r"[kqKQ]{1,4}|-").unwrap().is_match(&castling) {
            return Err(FenError::InvalidNotation);
        };

        let target = data.next().unwrap().to_owned();
        if !Regex::new(r"[A-Za-z][1-9]|-").unwrap().is_match(&target) {
            return Err(FenError::InvalidNotation);
        };

        let halfmove_clock = data.next().unwrap().parse()?;
        let move_number = data.next().unwrap().parse()?;

        Ok(Fen {
            piece_data,
            active_color,
            castling,
            en_passant_target: Some(target),
            halfmove_clock,
            move_number,
        })
    }
}

#[derive(Debug)]
pub(crate) enum FenError {
    InvalidNotation,
    InvalidPieceSet,
    PieceNotFound,
}
impl Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenError::InvalidNotation => write!(f, "FEN is invalid"),
            FenError::InvalidPieceSet => write!(f, "PieceSet is invalid"),
            FenError::PieceNotFound => write!(f, "Piece not found in PieceSet"),
        }
    }
}
impl Error for FenError {}
impl From<ParseIntError> for FenError {
    fn from(_: ParseIntError) -> Self {
        FenError::InvalidNotation
    }
}
