use bevy::prelude::*;
mod board;
mod input;
mod tile;

use board::*;
use tile::*;

pub struct BoardPlugin;
pub use board::BoardSettings;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<RevealTileEvent>()
            .add_event::<MarkTileEvent>()
            .insert_resource(board::Board::default())
            .add_startup_system(create_board)
            .add_system(input::send_click_events)
            .add_system(reveal_tile)
            .add_system(mark_tile)
            .add_system(unmark_tile);
    }
}

fn create_board(mut commands: Commands, board_set: Res<BoardSettings>) {
    let (width, height) = board_set.board_size;
    let tile_size = board_set.tile_size;
    let bombs = Board::gen_bombs((width, height), board_set.bomb_count);

    let offset = Vec3::new(
        ((width - 1) as f32) * tile_size / 2.,
        ((height - 1) as f32) * tile_size / 2.,
        0.,
    );

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(
                width as f32 * tile_size,
                height as f32 * tile_size,
            )),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });

    for y in 0..height {
        for x in 0..width {
            let pos = Vec3::new((x as f32) * tile_size, (y as f32) * tile_size, 2.);
            let mut entity = commands.spawn();
            let coord = Coord { x, y };
            let mut tile = Tile::Empty;

            if bombs.contains(&coord) {
                tile = Tile::Bomb;
                entity.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(tile_size - 1., tile_size - 1.)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(pos - offset),
                    ..Default::default()
                });
            } else if let Some(count) = count_neighbour_bombs(&bombs, &coord) {
                tile = Tile::BombNeighbour(count);
                entity.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::YELLOW,
                        custom_size: Some(Vec2::new(tile_size - 1., tile_size - 1.)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(pos - offset),
                    ..Default::default()
                });
            } else {
                entity.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::DARK_GRAY,
                        custom_size: Some(Vec2::new(tile_size - 1., tile_size - 1.)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(pos - offset),
                    ..Default::default()
                });
            }

            entity.with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GRAY,
                            custom_size: Some(Vec2::new(tile_size - 1., tile_size - 1.)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..Default::default()
                    })
                    .insert(coord)
                    .insert(tile)
                    .insert(Name::new("Tile"));
            });
        }
    }
}

fn count_neighbour_bombs(bombs: &[Coord], coord: &Coord) -> Option<u8> {
    let mut count = 0;

    for neighbour in coord.neighbours() {
        if bombs.contains(&neighbour) {
            count += 1;
        }
    }

    if count > 0 {
        Some(count)
    } else {
        None
    }
}
