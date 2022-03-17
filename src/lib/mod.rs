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
            .add_startup_system(create_board)
            .add_system(input::send_click_events)
            .add_system(reveal_tile)
            .add_system(mark_tile)
            .add_system(unmark_tile);
    }
}

fn create_board(mut commands: Commands, board_set: Res<BoardSettings>, assets: Res<AssetServer>) {
    let (width, height) = board_set.board_size;
    let tile_size = board_set.tile_size;
    let bombs = gen_bombs((width, height), board_set.bomb_count);

    let offset = Vec3::new(
        ((width - 1) as f32) * tile_size / 2.,
        ((height - 1) as f32) * tile_size / 2.,
        0.,
    );

    let covered_tile_asset = assets.load("covered_tile.png");
    let bombs_asset = assets.load("bomb.png");
    let font = assets.load("basis33.regular.ttf");

    let text_style = TextStyle {
        font,
        font_size: tile_size - (tile_size / 3.),
        color: Color::rgb_u8(255, 220, 1),
        // color: Color::rgba_u8(9, 240, 120, 255),
    };
    let text_alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Center,
    };

    for y in 0..height {
        for x in 0..width {
            let pos = Vec3::new((x as f32) * tile_size, (y as f32) * tile_size, 1.);
            let mut entity = commands.spawn();
            let coord = Coord { x, y };
            let mut tile = Tile::Empty;

            entity.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(tile_size - 5., tile_size - 5.)),
                    color: Color::rgb_u8(40, 50, 65),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos - offset),
                ..Default::default()
            });

            if bombs.contains(&coord) {
                tile = Tile::Bomb;
                entity.with_children(|parent| {
                    parent.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..Default::default()
                        },
                        texture: bombs_asset.clone(),
                        ..Default::default()
                    });
                });
            } else if let Some(count) = count_neighbour_bombs(&bombs, &coord) {
                tile = Tile::BombNeighbour(count);
                entity.with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            count.to_string(),
                            text_style.clone(),
                            text_alignment,
                        ),
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..Default::default()
                    });
                });
            }

            entity.with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..Default::default()
                        },
                        texture: covered_tile_asset.clone(),
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
