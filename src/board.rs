use crate::piece::Piece;
use bevy::{math::IVec2, prelude::Color};

pub(crate) struct Board {
    generator: BoardGenerator,
    tiles: Vec<Tile>,
    pieces: Vec<Piece>,
}

impl Board {
    pub(crate) fn new(generator: BoardGenerator) -> Self {
        Board {
            generator,
            tiles: todo!(),
            pieces: todo!(),
        }
    }
}

pub(crate) struct Tile {
    color: Color,
}

pub(crate) struct BoardGenerator {}
impl BoardGenerator {
    pub(crate) fn new(dimensions: IVec2) -> Self {
        BoardGenerator {}
    }
}
