use super::{Coord, Tile};
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::{distributions::Uniform, thread_rng, Rng};

pub struct BoardSettings {
    pub tile_size: f32,
    pub board_size: (Width, Height),
    pub bomb_count: u16,
}

type Width = u16;
type Height = u16;

#[derive(Default)]
pub struct Board {
    pub covered_tiles: HashMap<Coord, (Tile, Entity)>,
}

impl Board {
    pub fn gen_bombs(board_size: (u16, u16), bomb_count: u16) -> Vec<Coord> {
        let (width, height) = board_size;

        let mut bombs = Vec::<Coord>::new();
        let dist = TupleUniform::new(&(0, 0), &(width, height));

        fill_rand_unique_coords(&dist, &mut bombs, bomb_count.into());

        bombs
    }
}

fn fill_rand_unique_coords(dist: &TupleUniform, vec: &mut Vec<Coord>, size: usize) {
    let mut map = HashMap::<(u16, u16), ()>::default();

    while map.len() < size {
        let x = thread_rng().sample(dist.0);
        let y = thread_rng().sample(dist.1);

        if map.insert((x, y), ()).is_none() {
            vec.push(Coord { x, y });
        }
    }
}

struct TupleUniform(Uniform<u16>, Uniform<u16>);

impl TupleUniform {
    pub fn new(low: &(u16, u16), high: &(u16, u16)) -> Self {
        Self(Uniform::new(low.0, high.0), Uniform::new(low.1, high.1))
    }
}
