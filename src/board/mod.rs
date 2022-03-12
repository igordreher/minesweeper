use bevy::prelude::*;
mod tile;

pub use tile::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(create_board);
    }
}

fn create_board(mut commands: Commands, map: Res<TileMap>) {
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
                .insert(tile::Tile::Empty);
        }
    }
}
