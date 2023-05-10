use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::camera::CameraPlugin;
use crate::current_position::CurrentPositionPlugin;
use crate::line_of_sight::LineOfSightPlugin;
use crate::map::MapPlugin;
use crate::mouse_cursor::MouseCoursorPlugin;

mod camera;
mod map;
mod constants;
mod mouse_cursor;
mod current_position;
mod line_of_sight;

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
        .add_plugin(MouseCoursorPlugin)
        .add_plugin(CurrentPositionPlugin)
        .add_plugin(LineOfSightPlugin)
        .run()
}
