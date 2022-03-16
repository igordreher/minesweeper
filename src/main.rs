use bevy::prelude::*;
mod lib;

use lib::input;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    .insert_resource(lib::BoardSettings {
        board_size: (10, 10),
        tile_size: 50.,
        bomb_count: 10,
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(lib::BoardPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(camera_setup);
    app.add_system(input::send_click_events);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
