use core::panic;

use crate::{
    notation::{Fen, NotationError},
    piece::{ChessPiece, Piece},
};
use bevy::prelude::*;

pub(crate) struct Board {
    descriptor: BoardDescriptor,
    pieces: Option<Vec<Piece>>,
}
impl Board {
    /// Creates a new [`Board`] based on [`BoardDescriptor`].
    pub(crate) fn new(descriptor: BoardDescriptor, pieces: Option<Vec<Piece>>) -> Self {
        Board { descriptor, pieces }
    }

    /// Places pieces on the board based on the provided Forsyth–Edwards Notation (FEN) string.
    /// # Errors
    ///
    /// This function will return an error if prvided FEN is not valid
    pub(crate) fn from_fen(&mut self, fen: &Fen) -> Result<(), NotationError> {
        let mut data = fen.piece_data.split("/");
        let mut row = 0;
        let mut col = 0;
        let mut pieces = Vec::new();
        for piece_data in data {
            for c in piece_data.chars() {
                if c.is_digit(10) {
                    col += c.to_digit(10).unwrap() as i32;
                } else {
                    let piece = Piece::chess(match ChessPiece::from_char(&c) {
                        Ok(piece) => piece,
                        Err(err) => return Err(err),
                    });
                    pieces.push(piece);
                    col += 1;
                }
            }
            row -= 1;
            col = 0;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct Tile {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
}
impl Tile {
    pub(crate) fn new(material: Handle<StandardMaterial>, mesh: Handle<Mesh>) -> Tile {
        Tile { material, mesh }
    }
}

pub(crate) enum BoardError {
    InvalidBoardDescriptor,
}

pub(crate) trait BoardGenerator {
    fn generate(descriptor: BoardDescriptor, commands: &mut Commands) -> Result<Board, BoardError>;
}

pub(crate) struct CheckersBoardGenerator {}
impl BoardGenerator for CheckersBoardGenerator {
    fn generate(
        descriptor: BoardDescriptor,
        mut commands: &mut Commands,
    ) -> Result<Board, BoardError> {
        let mut tiles = descriptor.tiles.iter().cycle();
        let first_tile = tiles.next().ok_or(BoardError::InvalidBoardDescriptor)?;
        let second_tile = tiles.next().ok_or(BoardError::InvalidBoardDescriptor)?;

        for i in 0..descriptor.dimensions.x {
            for j in 0..descriptor.dimensions.y {
                let tile = if (i + j + 1) % 2 == 0 {
                    first_tile.clone()
                } else {
                    second_tile.clone()
                };

                commands.spawn_bundle(PbrBundle {
                    mesh: tile.mesh.clone(),
                    material: tile.material.clone(),
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                });
            }
        }
        Ok(Board::new(descriptor, None))
    }
}

pub(crate) struct BoardDescriptor {
    dimensions: IVec2,
    tiles: Vec<Tile>,
}
impl BoardDescriptor {
    pub(crate) fn new(dimensions: IVec2, tiles: Vec<Tile>) -> Self {
        BoardDescriptor { dimensions, tiles }
    }

    pub(crate) fn dimensions(&self) -> IVec2 {
        self.dimensions
    }
}
