use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::pfd::PfdPlugin;

mod pfd;
mod debug;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Navistar".to_owned(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
    }));

    app.add_plugins(PfdPlugin);

    #[cfg(debug_assertions)] // debug/dev builds only
    {
        app.add_plugins(crate::debug::fps_counter::FPSCounterPlugin);
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
