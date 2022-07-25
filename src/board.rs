use core::panic;

use crate::piece::Piece;
use bevy::prelude::*;
pub(crate) struct Board {
    descriptor: BoardDescriptor,
    pieces: Option<Vec<Piece>>,
}

impl Board {
    pub(crate) fn new(descriptor: BoardDescriptor, pieces: Option<Vec<Piece>>) -> Self {
        Board {
            descriptor,
            pieces,
        }
    }

    pub(crate) fn generate(&self, mut commands: Commands) {
        if self.descriptor.tiles.is_empty() {
            panic!("Tiles vec is empty, cannot generate board!")
        }
    }
}

#[derive(Clone)]
pub(crate) struct Tile {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
}
impl Tile {
    pub(crate) fn new(material: Handle<StandardMaterial>, mesh: Handle<Mesh>) -> Tile {
        Tile { 
            material, 
            mesh 
        }
    }
}

pub(crate) trait BoardGenerator {
    fn generate(descriptor: BoardDescriptor, commands: Commands) -> Board;
}

pub(crate) struct CheckersBoardGenerator {}
impl BoardGenerator for CheckersBoardGenerator {
    fn generate(descriptor: BoardDescriptor, mut commands: Commands) -> Board {
        let mut tiles = descriptor.tiles.iter().cycle();
        let first_tile = tiles
            .next()
            .expect("Not Enough tiles in BoardDescriptor for Checkers Pattern!");
        let second_tile = tiles
            .next()
            .expect("Not Enough tiles in BoardDescriptor for Checkers Pattern!");
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
        Board::new(descriptor, None)
    }
}

pub(crate) struct BoardDescriptor {
    dimensions: IVec2,
    tiles: Vec<Tile>,
}
impl BoardDescriptor {
    pub(crate) fn new(dimensions: IVec2, tiles: Vec<Tile>) -> Self {
        BoardDescriptor {
            dimensions,
            tiles, 
        }
    }

    pub(crate) fn dimensions(&self) -> IVec2 {
        self.dimensions
    }
}
