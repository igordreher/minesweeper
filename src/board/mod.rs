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
    let offset = 5.0;
    let mut sprites = vec![];

    for y in 0..tile_map.height {
        for x in 0..tile_map.width {
            sprites.push(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(tile_size - offset, tile_size - offset)),
                    ..Default::default()
                },
                transform: Transform::from_xyz((x as f32) * tile_size, (y as f32) * tile_size, 1.),
                ..Default::default()
            })
        }
    }

    commands.spawn_batch(sprites);
}
