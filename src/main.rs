use bevy::prelude::*;
mod board;

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
    .insert_resource(board::TileMap {
        height: 10,
        width: 10,
        bombs: 10,
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(board::BoardPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
