use bevy::prelude::*;

#[derive(Debug, Component)]
pub enum Tile {
    Empty,
    Bomb,
    BombNeighbour(u8),
}

#[derive(Debug, Component, PartialEq)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
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

pub fn reveal_tile(mut rev_ev: EventReader<RevealTileEvent>, q: Query<(&Coord, &Tile)>) {
    for ev in rev_ev.iter() {
        for (coord, tile) in q.iter() {
            if (&ev.0 == coord) {
                #[cfg(feature = "debug")]
                {
                    println!("{:?}", coord);
                    println!("{:?}", tile);
                }
            }
        }
    }
}

pub struct RevealTileEvent(pub Coord);
pub struct MarkTileEvent(pub Coord);

impl From<Vec2> for Coord {
    fn from(vec: Vec2) -> Self {
        let vec = vec.floor();
        Self {
            x: (vec.x as u16),
            y: (vec.y as u16),
        }
    }
}
