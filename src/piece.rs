use bevy::{prelude::*, reflect::List, utils::HashSet};

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
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}
impl Piece {
    pub(crate) fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Piece {
            transform: TransformBundle::identity(),
            mesh,
            material,
        }
    }

    pub(crate) fn chess(
        piece: ChessPiece,
        material: Handle<StandardMaterial>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let path = match piece {
            ChessPiece::King => "pieces/pieces.glb#Mesh0/Primitive0",
            ChessPiece::Pawn => "pieces/pieces.glb#Mesh1/Primitive0",
            ChessPiece::Knight => "pieces/pieces.glb#Mesh2/Primitive0",
            ChessPiece::Rook => "pieces/pieces.glb#Mesh3/Primitive0",
            ChessPiece::Bishop => "pieces/pieces.glb#Mesh4/Primitive0",
            ChessPiece::Queen => "pieces/pieces.glb#Mesh5/Primitive0",
        };
        let mesh = asset_server.load(path);
        Piece::new(mesh, material)
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
            mesh: self.mesh.clone(),
            material: self.material.clone(),
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
