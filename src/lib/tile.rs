use super::board::Board;
use bevy::prelude::*;

#[derive(Debug)]
pub enum Tile {
    Empty,
    Bomb,
    BombNeighbour(u8),
}

#[derive(Debug, Component, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

pub fn reveal_tile(
    mut commands: Commands,
    mut rev_ev: EventReader<RevealTileEvent>,
    mut board: ResMut<Board>,
) {
    for ev in rev_ev.iter() {
        find_tiles_to_uncover(&mut commands, &ev.0, &mut board);
    }
}

fn find_tiles_to_uncover(commands: &mut Commands, coord: &Coord, board: &mut Board) {
    if let Some((tile, e)) = board.covered_tiles.remove(coord) {
        match tile {
            Tile::Empty => {
                (*commands).entity(e).despawn();
                for neighbour in coord.neighbours() {
                    find_tiles_to_uncover(commands, &neighbour, board);
                }
            }
            _ => {
                (*commands).entity(e).despawn();
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

impl Coord {
    pub fn neighbours(&self) -> Vec<Self> {
        let mut vec = Vec::new();
        for x in self.x.saturating_sub(1)..self.x + 2 {
            vec.push(Self {
                x,
                y: self.y.saturating_sub(1),
            });
            vec.push(Self { x, y: self.y + 1 });
            if x != self.x {
                vec.push(Self { x, y: self.y });
            }
        }
        vec
    }
}
