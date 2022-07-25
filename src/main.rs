mod ascii;
mod debug;
mod player;
mod tilemap;

use crate::ascii::Ascii;
use crate::debug::DebugPlugin;
use crate::player::PlayerPlugin;
use crate::tilemap::TileMapPlugin;
use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TITLE_SIZE: f32 = 0.1;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 900.0,
            height: 600.0,
            title: "Bevy Tutorial".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Ascii)
        .add_startup_system(spawn_camera)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TileMapPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
