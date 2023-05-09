use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::camera::CameraPlugin;
use crate::map::MapPlugin;

mod camera;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800.0, 600.0).into(),
                        title: "shadowcasting".to_string(),
                        resizable: false,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        .add_plugin(CameraPlugin)
        .add_plugin(MapPlugin)
        .run()
}
