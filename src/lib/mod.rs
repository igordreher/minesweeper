use bevy::{prelude::*, utils::HashMap};
mod board;
pub mod input;
mod tile;

use board::Board;
pub use tile::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<RevealTileEvent>()
            .add_event::<MarkTileEvent>()
            .insert_resource(board::Board {
                covered_tiles: HashMap::default(),
            })
            .add_startup_system(create_board)
            .add_system(reveal_tile);
    }
}

fn create_board(mut commands: Commands, map: Res<TileMap>, mut board_res: ResMut<Board>) {
    let tile_size = 50.0;
    let offset = Vec3::new(
        ((map.width - 1) as f32) * tile_size / 2.,
        ((map.height - 1) as f32) * tile_size / 2.,
        1.,
    );

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = Vec3::new((x as f32) * tile_size, (y as f32) * tile_size, 1.);
            let mut entity = commands.spawn();
            let coord = Coord { x, y };

            entity
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: Some(Vec2::new(tile_size - 1., tile_size - 1.)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(pos - offset),
                    ..Default::default()
                })
                .insert(Tile::Empty)
                .insert(coord);

            board_res
                .covered_tiles
                .insert(coord, (Tile::Empty, entity.id()));
        }
    }
}
