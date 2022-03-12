use bevy::prelude::Color;

pub enum Tile {
    Empty,
    Bomb,
    BombNeighbour(u8),
}

pub struct TileMap {
    pub width: u32,
    pub height: u32,
    bombs: u32,
    pub tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new(width: u32, height: u32, bombs: u32) -> Self {
        let tiles = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();

        Self {
            width,
            height,
            bombs,
            tiles,
        }
    }
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
