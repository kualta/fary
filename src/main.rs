#[allow(unused, dead_code)]
mod board;
mod piece;
mod fen;

use bevy::prelude::*;
use board::*;
use piece::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Board Editor".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_setup)
        .add_startup_system(generate_board)
        .add_startup_system(place_pieces)
        .run();
}

fn camera_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn generate_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

    let mut tiles = Vec::new();
    tiles.push(Tile::new(white_material, mesh.clone()));
    tiles.push(Tile::new(black_material, mesh.clone()));

    let board_descriptor = BoardDescriptor::new(IVec2::new(8, 8), tiles);

    let board = CheckersBoardGenerator::generate(board_descriptor, commands);
}

fn place_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let white_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let black_material = materials.add(Color::rgb(0.0, 0.0, 0.0).into());

    let pawn = Piece::chess(ChessPiece::Pawn, white_material.clone(), &asset_server)
        .spawn_at(Vec3::new(1., 0., 0.), &mut commands);
    let rook = Piece::chess(ChessPiece::Rook, white_material.clone(), &asset_server)
        .spawn_at(Vec3::new(0., 0., 0.), &mut commands);
    let knight = Piece::chess(ChessPiece::Knight, white_material.clone(), &asset_server)
        .spawn_at(Vec3::new(0., 0., 1.), &mut commands);
    let bishop = Piece::chess(ChessPiece::Bishop, white_material.clone(), &asset_server)
        .spawn_at(Vec3::new(0., 0., 2.), &mut commands);
    let queen = Piece::chess(ChessPiece::Queen, white_material.clone(), &asset_server)
        .spawn_at(Vec3::new(0., 0., 3.), &mut commands);
    let king = Piece::chess(ChessPiece::King, white_material.clone(), &asset_server)
        .spawn_at(Vec3::new(0., 0., 4.), &mut commands);
}
