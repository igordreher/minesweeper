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
) {
    for ev in events.iter() {
        #[cfg(feature = "debug")]
        println!("Revealing tile {:?}", ev.0);

        let mut map = tiles
            .iter()
            .map(|tile| (tile.1, (tile.0, tile.2)))
            .collect();

        find_tiles_to_uncover(&mut commands, &ev.0, &mut map);
    }
}

fn find_tiles_to_uncover(
    commands: &mut Commands,
    coord: &Coord,
    covered_tiles: &mut HashMap<&Coord, (&Tile, Entity)>,
) {
    if let Some((tile, e)) = covered_tiles.remove(coord) {
        match tile {
            Tile::Empty => {
                (*commands).entity(e).despawn();
                for neighbour in coord.neighbours() {
                    find_tiles_to_uncover(commands, &neighbour, covered_tiles);
                }
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
) {
    for event in mark_ev.iter() {
        for (coord, entity) in tiles.iter() {
            if &event.0 == coord {
                #[cfg(feature = "debug")]
                println!("Marking tile {:?}", event.0);
                // let flag = commands
                //     .spawn_bundle(SpriteBundle {
                //         sprite: Sprite {
                //             color: Color::BLUE,
                //             custom_size: Some(Vec2::new(10., 10.)),
                //             ..Default::default()
                //         },
                //         transform: Transform::from_xyz(0., 0., 2.),
                //         ..Default::default()
                //     })
                //     .insert(Name::new("Flag"))
                //     .id();

                commands
                    .entity(entity)
                    .insert(MarkedTile)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLUE,
                                    custom_size: Some(Vec2::new(10., 10.)),
                                    ..Default::default()
                                },
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
