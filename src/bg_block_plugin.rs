use crate::component::Moveable;
use crate::component::*;
use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};
use std::collections::LinkedList;
use std::ops::Range;

pub struct BgBlockPlugin;

impl Plugin for BgBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Wireframe2dPlugin::default());
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let game = BData::new();

    let mut rc = 0;
    for r in &game.blocks {
        let mut cc = 0;
        let x = -BLOCK_WIDTH / 2 as f32 + BLOCK_PER_WIDTH * rc as f32 + BLOCK_PER_WIDTH / 2 as f32;
        rc = rc + 1;
        for c in r {
            let color = Color::srgba(0.8, 0.8, 0.8, 0.5);
            let y = -BLOCK_HEIGHT / 2 as f32
                + BLOCK_PER_HEIGHT * cc as f32
                + BLOCK_PER_HEIGHT / 2 as f32;
            cc = cc + 1;
            let rect = Rectangle::new(BLOCK_PER_WIDTH - 1 as f32, BLOCK_PER_HEIGHT - 1 as f32);
            let shape = meshes.add(rect);
            commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(x, y, 0.0),
                DefaultCell,
            ));
        }
    }
}

fn toggle_wireframe(
    mut commands: Commands,
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<Entity, With<DefaultCell>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }

    if keyboard.just_pressed(KeyCode::KeyW) {
        for k in query {
            commands.entity(k).despawn()
        }
    }
}
