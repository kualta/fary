#[allow(unused, dead_code)]

mod board;
mod piece;

use board::*;
use piece::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(generate_board)
        .run();
}

fn generate_board() {
    let board_generator = BoardGenerator::new(IVec2::new(8, 8));
    let tiles = Tile::new();
    let mut pieces = Vec::new();
    pieces.push(Piece::new());

    let board = Board::new(board_generator, tiles, pieces);
}
