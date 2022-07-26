pub(crate) struct Fen {
    piece_data: String,
    active_color: String,
    castling: String,
    en_passant: String,
    halfmove_clock: i32,
    move_number: i32,
}
impl Fen {
    pub(crate) fn new(input: &str) -> Result<Fen, FenError> {
        let mut data = input.split_whitespace();

        if data.clone().count() != 6 {
            return Err(FenError::InvalidNotation);
        };

        // FIXME: Check before unwraping for better errors
        Ok(Fen {
            piece_data: data.next().unwrap().to_owned(),
            active_color: data.next().unwrap().to_owned(),
            castling: data.next().unwrap().to_owned(),
            en_passant: data.next().unwrap().to_owned(),
            halfmove_clock: data.next().unwrap().parse().unwrap(),
            move_number: data.next().unwrap().parse().unwrap(),
        })
    }
}

pub(crate) enum FenError {
    InvalidNotation,
    InvalidPieceSet,
    PieceNotFound,
}
