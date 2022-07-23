#[allow(unused, dead_code)]
mod board;
mod piece;

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
    let board_descriptor = BoardDescriptor::new(IVec2::new(8, 8));
    let mut pieces = Vec::new();
    pieces.push(Piece::new());


    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let white_material = materials.add(Color::rgb(0.9, 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

    let mut tiles = Vec::new();
    tiles.push(Tile::new(white_material, mesh.clone()));
    tiles.push(Tile::new(black_material, mesh.clone()));

    let board = Board::new(board_descriptor, tiles, pieces);

    board.generate(commands)
}
