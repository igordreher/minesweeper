use super::{Coord, Tile};
use bevy::{prelude::Entity, utils::HashMap};

pub struct Board {
    pub covered_tiles: HashMap<Coord, (Tile, Entity)>,
}
