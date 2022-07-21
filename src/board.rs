use crate::piece::Piece;
use bevy::{math::IVec2, prelude::Color, utils::hashbrown::hash_map::IntoIter};

pub(crate) struct Board {
    generator: BoardGenerator,
    tiles: Vec<Tile>,
    pieces: Vec<Piece>,
}

impl Board {
    pub(crate) fn new(generator: BoardGenerator, tiles: Vec<Tile>, pieces: Vec<Piece>) -> Self {
        Board {
            generator,
            tiles,
            pieces,
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

    pub(crate) fn place_pieces() {}
}
impl Tile {
    pub(crate) fn new() -> Vec<Tile> {
        todo!()
    }
}
