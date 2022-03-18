use crate::lib::*;
use bevy::input::{mouse::MouseButtonInput, ElementState};

pub fn send_click_events(
    windows: Res<Windows>,
    mut click: EventReader<MouseButtonInput>,
    mut rev_ev: EventWriter<RevealTileEvent>,
    mut mark_ev: EventWriter<MarkTileEvent>,
    state: Res<BoardState>,
) {
    if state.game_over {
        return;
    }
    let window = windows.get_primary().unwrap();

    for ev in click.iter() {
        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let coord = screen_to_coord(screen_pos, window_size);

            if ev.state == ElementState::Pressed {
                match ev.button {
                    MouseButton::Left => {
                        rev_ev.send(RevealTileEvent(coord));
                    }
                    MouseButton::Right => {
                        #[cfg(feature = "debug")]
                        println!("sending {:?}", coord);

                        mark_ev.send(MarkTileEvent(coord));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn screen_to_coord(screen_pos: Vec2, window_size: Vec2) -> Coord {
    let x_offset = (window_size.x - 500.) / 2.;
    let y_offset = (window_size.y - 500.) / 2.;
    let offset = Vec2::new(x_offset, y_offset);
    let board_pos = screen_pos - offset;
    let coord = board_pos / 50.;

    coord.into()
}
