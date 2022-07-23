use core::panic;

use crate::piece::Piece;
use bevy::{
    math::{IVec2, Vec3},
    pbr::{Material, PbrBundle, StandardMaterial},
    prelude::{Color, Commands, Handle, Mesh, Transform},
    utils::hashbrown::hash_map::IntoIter,
};

pub(crate) struct Board {
    descriptor: BoardDescriptor,
    tiles: Vec<Tile>,
    pieces: Vec<Piece>,
}

impl Board {
    pub(crate) fn new(generator: BoardDescriptor, tiles: Vec<Tile>, pieces: Vec<Piece>) -> Self {
        Board {
            descriptor: generator,
            tiles,
            pieces,
        }
    }

    pub(crate) fn generate(&self, mut commands: Commands) {
        if self.tiles.is_empty() {
            panic!("Tiles vec is empty, cannot generate board!")
        }

        // TODO: Add pattern generation
        let mut tiles = self.tiles.iter().cloned().cycle();
        for i in 0..self.descriptor.dimensions.x {
            for j in 0..self.descriptor.dimensions.y {
                let tile = tiles
                    .next()
                    .expect("Next tile not found?! This should never happen.");
                commands.spawn_bundle(PbrBundle {
                    mesh: tile.mesh.clone(),
                    material: tile.material.clone(),
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                });
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct Tile {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
}

pub(crate) struct BoardDescriptor {
    dimensions: IVec2,
}
impl BoardDescriptor {
    pub(crate) fn new(dimensions: IVec2) -> Self {
        BoardDescriptor { dimensions }
    }

    pub(crate) fn dimensions(&self) -> IVec2 {
        self.dimensions
    }
}
impl Tile {
    pub(crate) fn new(material: Handle<StandardMaterial>, mesh: Handle<Mesh>) -> Tile {
        Tile {
            material,
            mesh
        }
    }
}
