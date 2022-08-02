#[allow(unused, dead_code)]
mod board;
mod piece;
mod notation;

use bevy::prelude::*;
use board::*;
use notation::Fen;
use piece::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Board Editor".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_setup)
        .add_startup_system_to_stage(StartupStage::PreStartup, generate_board)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_pieces)
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

    let board = match CheckersBoardGenerator::generate(board_descriptor, &mut commands) {
        Ok(board) => board,
        Err(err) => panic!("{}", err),
    };
    commands.insert_resource(board);
}

fn place_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut board: ResMut<Board>,
) {
    let white_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let black_material = materials.add(Color::rgb(0.0, 0.0, 0.0).into());

    let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
    board.from_fen(&Fen::from_raw(fen).unwrap());
    // let knight = Piece::chess(ChessPiece::Knight, white_material.clone(), &asset_server)
    //     .spawn_at(Vec3::new(0., 0., 1.), &mut commands);
    // let bishop = Piece::chess(ChessPiece::Bishop, white_material.clone(), &asset_server)
    //     .spawn_at(Vec3::new(0., 0., 2.), &mut commands);
    // let queen = Piece::chess(ChessPiece::Queen, white_material.clone(), &asset_server)
    //     .spawn_at(Vec3::new(0., 0., 3.), &mut commands);
    // let king = Piece::chess(ChessPiece::King, white_material.clone(), &asset_server)
    //     .spawn_at(Vec3::new(0., 0., 4.), &mut commands);
    
}

fn load_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let meshes = ChessMesh {
        pawn: asset_server.load("pieces/pieces.glb#Mesh0/Primitive0"),
        knight: asset_server.load("pieces/pieces.glb#Mesh1/Primitive0"),
        bishop: asset_server.load("pieces/pieces.glb#Mesh2/Primitive0"),
        rook: asset_server.load("pieces/pieces.glb#Mesh3/Primitive0"),
        queen: asset_server.load("pieces/pieces.glb#Mesh4/Primitive0"),
        king: asset_server.load("pieces/pieces.glb#Mesh5/Primitive0"),
    };
    commands.insert_resource(meshes);
}
