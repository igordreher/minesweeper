use bevy::prelude::*;
mod tile;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(create_board);
    }
}

fn create_board(mut commands: Commands) {
    let tile_map = tile::TileMap::new(10, 10, 5);

    let tile_size = 50.0;
    let offset = Vec3::new(
        ((tile_map.width - 1) as f32) * tile_size / 2.,
        ((tile_map.height - 1) as f32) * tile_size / 2.,
        1.,
    );
    let mut sprites = vec![];

    for y in 0..tile_map.height {
        for x in 0..tile_map.width {
            let pos = Vec3::new((x as f32) * tile_size, (y as f32) * tile_size, 1.);
            sprites.push(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(tile_size - 1., tile_size - 1.)),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos - offset),
                ..Default::default()
            })
        }
    }

    commands.spawn_batch(sprites);
}
