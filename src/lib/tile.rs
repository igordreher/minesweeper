use super::board::*;
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Component)]
pub enum Tile {
    Empty,
    Bomb,
    BombNeighbour(u8),
}

#[derive(Debug, Component, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

pub fn reveal_tile(
    mut commands: Commands,
    tiles: Query<(&Tile, &Coord, Entity), Without<MarkedTile>>,
    mut events: EventReader<RevealTileEvent>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    for ev in events.iter() {
        #[cfg(feature = "debug")]
        println!("Revealing tile {:?}", ev.0);

        let mut map = tiles
            .iter()
            .map(|tile| (tile.1, (tile.0, tile.2)))
            .collect();

        find_tiles_to_uncover(&mut commands, &ev.0, &mut map, &mut game_over_event);
    }
}

fn find_tiles_to_uncover(
    commands: &mut Commands,
    coord: &Coord,
    covered_tiles: &mut HashMap<&Coord, (&Tile, Entity)>,
    event: &mut EventWriter<GameOverEvent>,
) {
    if let Some((tile, e)) = covered_tiles.remove(coord) {
        match tile {
            Tile::Empty => {
                (*commands).entity(e).despawn();

                for neighbour in coord.neighbours() {
                    find_tiles_to_uncover(commands, &neighbour, covered_tiles, event);
                }
            }
            Tile::Bomb => {
                event.send(GameOverEvent(e));
            }
            _ => {
                (*commands).entity(e).despawn();
            }
        }
    }
}

pub fn mark_tile(
    mut commands: Commands,
    mut mark_ev: EventReader<MarkTileEvent>,
    tiles: Query<(&Coord, Entity), Without<MarkedTile>>,
    assets: Res<AssetServer>,
    board: Res<BoardSettings>,
) {
    let flag = assets.load("flag.png");

    for event in mark_ev.iter() {
        for (coord, entity) in tiles.iter() {
            if &event.0 == coord {
                #[cfg(feature = "debug")]
                println!("Marking tile {:?}", event.0);

                commands
                    .entity(entity)
                    .insert(MarkedTile)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(board.tile_size, board.tile_size)),
                                    ..Default::default()
                                },
                                texture: flag.clone(),
                                transform: Transform::from_xyz(0., 0., 2.),
                                ..Default::default()
                            })
                            .insert(Name::new("Flag"));
                    });
            }
        }
    }
}

pub fn unmark_tile(
    mut commands: Commands,
    mut mark_ev: EventReader<MarkTileEvent>,
    tiles: Query<(&Coord, Entity), With<MarkedTile>>,
) {
    for event in mark_ev.iter() {
        for (coord, entity) in tiles.iter() {
            if &event.0 == coord {
                #[cfg(feature = "debug")]
                println!("unmarking tile {:?}", event.0);

                commands
                    .entity(entity)
                    .remove::<MarkedTile>()
                    .despawn_descendants();
            }
        }
    }
}

pub struct RevealTileEvent(pub Coord);
pub struct MarkTileEvent(pub Coord);
pub struct GameOverEvent(pub Entity);

#[derive(Component)]
pub struct MarkedTile;

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
        for y in self.y.saturating_sub(1)..self.y + 2 {
            for x in self.x.saturating_sub(1)..self.x + 2 {
                if self.x == x && self.y == y {
                    continue;
                };
                vec.push(Self { x, y })
            }
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_bring_neighbours() {
        let coord = Coord { x: 1, y: 1 };
        let mut neighbours = coord.neighbours();
        let mut result = vec![
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 2 },
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: 2 },
            Coord { x: 2, y: 0 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 2 },
        ];

        neighbours.sort();
        result.sort();

        assert_eq!(neighbours, result);
    }

    #[test]
    fn should_not_bring_corners() {
        let coord = Coord { x: 0, y: 0 };
        let mut neighbours = coord.neighbours();
        let mut result = vec![
            Coord { x: 0, y: 1 },
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: 1 },
        ];

        neighbours.sort();
        result.sort();

        assert_eq!(neighbours, result);
    }
}
