use super::{Coord, Tile};
use bevy::{prelude::*, utils::HashMap};

pub struct BoardSettings {
    pub tile_size: f32,
    pub board_size: (u16, u16),
    pub bomb_count: u16,
}

#[derive(Default)]
pub struct Board {
    pub covered_tiles: HashMap<Coord, (Tile, Entity)>,
    pub tiles: Vec<Vec<Tile>>,
}

impl Board {
    pub fn gen_board() {}
}
