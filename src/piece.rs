use bevy::{prelude::*, reflect::List, utils::HashSet};

use crate::notation::NotationError;

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

pub(crate) struct ChessMesh {
    pub(crate) pawn: Handle<Mesh>,
    pub(crate) knight: Handle<Mesh>,
    pub(crate) bishop: Handle<Mesh>,
    pub(crate) rook: Handle<Mesh>,
    pub(crate) queen: Handle<Mesh>,
    pub(crate) king: Handle<Mesh>,
}

pub(crate) struct Piece {
    transform: TransformBundle,
    mesh: Option<Handle<Mesh>>,
    material: Option<Handle<StandardMaterial>>,
}
impl Piece {
    pub(crate) fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Piece {
            transform: TransformBundle::identity(),
            mesh: Some(mesh),
            material: Some(material),
        }
    }

    pub(crate) fn set_material(&mut self, material: Handle<StandardMaterial>) {
        self.material = Some(material);
    }

    pub(crate) fn set_mesh(&mut self, mesh: Handle<Mesh>) {
        self.mesh = Some(mesh);
    }

    pub(crate) fn from_char(c: char) -> Result<Piece, NotationError> {
        let piece = match c {
            'p' => ChessPiece::Pawn,
            'n' => ChessPiece::Knight,
            'b' => ChessPiece::Bishop,
            'r' => ChessPiece::Rook,
            'q' => ChessPiece::Queen,
            'k' => ChessPiece::King,
            _ => return Err(NotationError::InvalidPiece(c.to_string())),
        };
        Ok(Piece::chess(piece))
    }

    pub(crate) fn chess(
        piece: ChessPiece,
    ) -> Self {
        todo!();
        // let mesh = match piece {
        //     ChessPiece::Pawn => meshes.pawn,
        //     ChessPiece::Knight => meshes.knight,
        //     ChessPiece::Bishop => meshes.bishop,
        //     ChessPiece::Rook => meshes.rook,
        //     ChessPiece::Queen => meshes.queen,
        //     ChessPiece::King => meshes.king,
        // };
        // Piece::new(mesh, material)
    }

    pub(crate) fn shogi(piece: ShogiPiece) -> Self {
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
    }

    // FIXME: Add to board, then spawn in board coordinates
    pub(crate) fn spawn_at(&self, pos: Vec3, commands: &mut Commands) {
        commands.spawn_bundle(PbrBundle {
            mesh: self.mesh.as_ref().unwrap().clone(),
            material: self.material.as_ref().unwrap().clone(),
            transform: {
                let mut transform = Transform::from_translation(pos);
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2)); // FIXME: Unify all model scales
                transform
            },
            ..Default::default()
        });
    }
}

pub(crate) struct PieceSet {
    set: HashSet<Piece>
}
impl PieceSet {
    pub(crate) fn chess () {
        todo!();
    }

    pub(crate) fn shogi () {
        todo!();
    }
}
