mod bg_block_plugin;
mod component;
mod draw_plugin;
mod frame_plugin;
mod game_plugin;

use crate::bg_block_plugin::BgBlockPlugin;
use crate::component::*;
use bevy::asset::ErasedAssetLoader;
use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::window::WindowMode;
use bevy_kira_audio::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "俄罗斯方块".into(),
                mode: WindowMode::Fullscreen(
                    MonitorSelection::Primary,
                    VideoModeSelection::Current,
                ),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: bevy::window::WindowResolution::new(
                    (BLOCK_WIDTH + (BLOCK_WIDTH / 2.0)) as u32,
                    (BLOCK_HEIGHT + (BLOCK_HEIGHT / 2.0)) as u32,
                ),
                present_mode: bevy::window::PresentMode::Fifo, // 垂直同步模式
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(MinimalPlugins)
        // .add_plugins(AssetPlugin::default())
        .add_plugins(AudioPlugin)
        .add_plugins((frame_plugin::FramePlugin, bg_block_plugin::BgBlockPlugin))
        .add_plugins((game_plugin::GamePlugin, draw_plugin::DrawPlugin))
        .add_systems(Startup, start_background_audio)
        .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.set_volume(10.0);
    audio
        .play(asset_server.load("sounds/gysy-beyond.ogg"))
        .looped();
}
