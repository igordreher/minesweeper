use bevy::prelude::*;

#[derive(Component)]
pub enum Tile {
    Empty,
    Bomb,
    BombNeighbour(u8),
}

impl Tile {
    pub fn color(&self) -> Color {
        match self {
            Tile::Empty => Color::WHITE,
            Tile::Bomb => Color::RED,
            Tile::BombNeighbour(n) => Color::YELLOW,
        }
    }
}

pub struct TileMap {
    pub height: u16,
    pub width: u16,
    pub bombs: u16,
}
