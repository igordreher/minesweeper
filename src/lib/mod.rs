use bevy::prelude::*;
mod board;
pub mod input;
mod tile;

use board::*;
pub use tile::*;

pub struct BoardPlugin;
pub use board::BoardSettings;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<RevealTileEvent>()
            .add_event::<MarkTileEvent>()
            .insert_resource(board::Board::default())
            .add_startup_system(create_board)
            .add_system(reveal_tile);
    }
}

fn create_board(mut commands: Commands, board: Res<BoardSettings>, mut board_res: ResMut<Board>) {
    let (width, height) = board.board_size;

    let offset = Vec3::new(
        ((width - 1) as f32) * board.tile_size / 2.,
        ((height - 1) as f32) * board.tile_size / 2.,
        1.,
    );

    for y in 0..height {
        for x in 0..width {
            let pos = Vec3::new(
                (x as f32) * board.tile_size,
                (y as f32) * board.tile_size,
                1.,
            );
            let mut entity = commands.spawn();
            let coord = Coord { x, y };

            entity
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: Some(Vec2::new(board.tile_size - 1., board.tile_size - 1.)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(pos - offset),
                    ..Default::default()
                })
                .insert(coord);

            board_res
                .covered_tiles
                .insert(coord, (Tile::Empty, entity.id()));
        }
    }
}
