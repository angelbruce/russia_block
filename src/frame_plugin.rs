use crate::component::*;
use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::window::*;
use std::ops::Range;

pub struct FramePlugin;

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GState(GameState::Idle));
        app.add_systems(Startup, (setup, background_cell_setup).chain());
        app.add_systems(Update, update_event);
    }
}

fn update_event(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gstate: ResMut<GState>,
    mut data_panel: ResMut<DataPanel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Text, &Statable)>,
) {
    if (keyboard.just_pressed(KeyCode::Enter)
        && gstate.0 != GameState::Running
        && gstate.0 != GameState::Pause)
    {
        gstate.0 = GameState::Running;
        let row_range = Range {
            start: 0,
            end: BLOCK_Y_COUNT as usize,
        };
        for row in row_range {
            let col_range = Range {
                start: 0,
                end: BLOCK_X_COUNT as usize,
            };
            for col in col_range {
                data_panel.blocks[row][col] = 0;
            }
        }
        for (entity, _, _) in query {
            commands.entity(entity).despawn();
        }
    } else if (keyboard.just_pressed(KeyCode::Space)) {
        if gstate.0 == GameState::Pause {
            gstate.0 = GameState::Running;
        } else if gstate.0 == GameState::Running {
            gstate.0 = GameState::Pause
        }
    } else if (keyboard.just_pressed(KeyCode::Escape)) {
        gstate.0 = GameState::Idle
    }

    match gstate.0 {
        GameState::Idle => {
            commands.spawn((
                Text::new("PRESS Enter TO START GAME!"),
                Node {
                    position_type: PositionType::Absolute,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    top: px(48),
                    left: px(12),
                    ..default()
                },
                Statable,
            ));
        }
        GameState::Pause => {}
        GameState::Running => {}
        GameState::Stop => {
            commands.spawn((
                Text::new("GAME OVER!"),
                Node {
                    position_type: PositionType::Absolute,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    top: px(48),
                    left: px(12),
                    ..default()
                },
                Statable,
            ));
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let shapes = [
        meshes.add(Rectangle::new(
            BLOCK_WIDTH + 80 as f32,
            BLOCK_HEIGHT + 80 as f32,
        )),
        meshes.add(Rectangle::new(BLOCK_WIDTH, BLOCK_HEIGHT)),
    ];

    let colors = [Color::srgba(0.3, 0.4, 0.7, 1.0), Color::srgb(0.5, 0.5, 0.5)];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = colors[i];

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    commands.spawn((
        Text::new("RUSSIA SQUARE BLOCK"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn background_cell_setup(
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
            let color = Color::srgba(
                BLOCK_DEF_COLOR_RED,
                BLOCK_DEF_COLOR_GREEN,
                BLOCK_DEF_COLOR_BLUE,
                1.0,
            );
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
