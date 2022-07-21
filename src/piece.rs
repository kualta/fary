use bevy::{prelude::Bundle, transform::{TransformBundle, self}};

pub(crate) enum ChessPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub(crate) enum ShogiPiece {
    Pawn,
    Lance,
    Knight,
    GoldGeneral,
    SilverGeneral,
    Bishop,
    Rook,
    KingC, // Challenging King
    KingR, // Reigning King
    PawnPromoted,
    LancePromoted,
    KnightPromoted,
    SilverPromoted,
    BishopPromoted,
    RookPromoted,
}

#[derive(Bundle)]
pub(crate) struct Piece {
    #[bundle]
    transform: TransformBundle,
}
impl Piece {
    pub(crate) fn new() -> Self {
        Piece {
            transform: TransformBundle::identity(),
        }
    }

    pub(crate) fn chess(piece: ChessPiece) -> Self {
        let new_piece = Piece::new();

        match piece {
            ChessPiece::Pawn => todo!(),
            ChessPiece::Knight => todo!(),
            ChessPiece::Bishop => todo!(),
            ChessPiece::Rook => todo!(),
            ChessPiece::Queen => todo!(),
            ChessPiece::King => todo!(),
        }

        new_piece
    }

    pub(crate) fn shogi(piece: ShogiPiece) -> Self {
        let new_piece = Piece::new();

        match piece {
            ShogiPiece::Pawn => todo!(),
            ShogiPiece::Lance => todo!(),
            ShogiPiece::Knight => todo!(),
            ShogiPiece::GoldGeneral => todo!(),
            ShogiPiece::SilverGeneral => todo!(),
            ShogiPiece::Bishop => todo!(),
            ShogiPiece::Rook => todo!(),
            ShogiPiece::KingC => todo!(),
            ShogiPiece::KingR => todo!(),
            ShogiPiece::PawnPromoted => todo!(),
            ShogiPiece::LancePromoted => todo!(),
            ShogiPiece::KnightPromoted => todo!(),
            ShogiPiece::SilverPromoted => todo!(),
            ShogiPiece::BishopPromoted => todo!(),
            ShogiPiece::RookPromoted => todo!(),
        }

        new_piece
    }
}
