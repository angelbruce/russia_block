use crate::component::*;
use bevy::prelude::*;
use bevy::reflect::Set;
use bevy_kira_audio::prelude::*;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::time::Duration;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DataPanel::new());
        app.insert_resource(DownTime(Timer::from_seconds(0.3, TimerMode::Repeating)));
        app.add_systems(
            Update,
            (
                (
                    direction_up,
                    direction_right,
                    direction_down,
                    direction_left,
                ),
                refresh,
                merge_data,
            )
                .chain(),
        );
    }
}

fn refresh(
    mut gstate: ResMut<GState>,
    mut data_panel: ResMut<DataPanel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<FixedTime>,
    mut down_timer: ResMut<DownTime>,
    mut commands: Commands,
    mut current_block: Query<(&mut Moveable, &mut Pos, &mut TransformBlock)>,
) {
    match gstate.0 {
        GameState::Pause | GameState::Idle | GameState::Stop => return,
        _ => {}
    }

    let mut current_exists = false;
    let down = down_timer.0.tick(time.delta()).just_finished();
    for (_, mut pos, mut block) in current_block {
        current_exists = true;
        let cur_pos = &mut pos as &mut Pos;
        if down {
            let block = &mut block as &mut TransformBlock;
            let collision = Collision {
                pos: cur_pos.clone(),
            };
            if collision.is_exist_when_translate(block, Vec2::new(0., 1.)) {
                return;
            }

            let next_pos = Pos::new(cur_pos.row + 1, cur_pos.col);
            let collision = Collision { pos: next_pos };
            if collision.is_exists_with_data_panel(&data_panel, block) {
                return;
            }

            pos.row += 1;
        }
    }

    if !current_exists {
        let block = TransformBlock::random();
        let pos = Pos::default_born();
        let collision = Collision { pos: pos.clone() };
        let cb = block.clone();
        commands.spawn((block, pos, Moveable));
        if collision.is_exists_with_data_panel(&data_panel, &cb) {
            gstate.0 = GameState::Stop;
            return;
        }
    }
}

fn direction_up(
    mut gstate: ResMut<GState>,
    mut data_panel: ResMut<DataPanel>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut Pos, &mut TransformBlock), With<Moveable>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    match gstate.0 {
        GameState::Pause | GameState::Idle | GameState::Stop => return,
        _ => {}
    }

    if (keyboard.just_pressed(KeyCode::ArrowUp)) {
        let effect = asset_server.load("sounds/plop.ogg");
        audio.set_volume(10.);
        audio.play(effect);

        for (entity, mut pos, mut transform_block) in query {
            let block = &mut transform_block as &mut TransformBlock;
            let cur_pos = &mut pos as &mut Pos;

            let typ = block.typ;
            let seed = (block.seed + 1) % 4;
            let new_block = block.rotate();
            let collision = Collision {
                pos: cur_pos.clone(),
            };
            if collision.is_exist_when_translate(&new_block, Vec2::new(0., 0.)) {
                return;
            }

            if collision.is_exists_with_data_panel(&data_panel, &new_block) {
                return;
            }

            commands.entity(entity).despawn();
            commands.spawn((new_block, pos.clone(), Moveable));
        }
    }
}

fn direction_left(
    mut gstate: ResMut<GState>,
    mut data_panel: ResMut<DataPanel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Pos, &mut TransformBlock), With<Moveable>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    match gstate.0 {
        GameState::Pause | GameState::Idle | GameState::Stop => return,
        _ => {}
    }

    if (keyboard.just_pressed(KeyCode::ArrowLeft)) {
        let effect = asset_server.load("sounds/plop.ogg");
        audio.set_volume(10.);
        audio.play(effect);

        for (mut pos, mut block) in query {
            let block_data = &mut block as &mut TransformBlock;
            let pos_data = &mut pos as &mut Pos;
            let collision = Collision {
                pos: pos_data.clone(),
            };
            if collision.is_exist_when_translate(&block_data, Vec2::new(-1., 0.)) {
                return;
            }

            if collision.is_exists_with_data_panel(&data_panel, &block_data) {
                return;
            }

            pos.col -= 1;
        }
    }
}

fn direction_right(
    mut gstate: ResMut<GState>,
    mut data_panel: ResMut<DataPanel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Moveable, &mut Pos, &mut TransformBlock), With<Moveable>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    match gstate.0 {
        GameState::Pause | GameState::Idle | GameState::Stop => return,
        _ => {}
    }

    if (keyboard.just_pressed(KeyCode::ArrowRight)) {
        let effect = asset_server.load("sounds/plop.ogg");
        audio.set_volume(10.);
        audio.play(effect);

        for (_, mut pos, mut block) in query {
            let block_data = &mut block as &mut TransformBlock;
            let pos_data = &mut pos as &mut Pos;
            let collision = Collision {
                pos: pos_data.clone(),
            };
            if collision.is_exist_when_translate(block_data, Vec2::new(1., 0.)) {
                return;
            }

            if collision.is_exists_with_data_panel(&data_panel, block_data) {
                return;
            }

            pos.col += 1;
        }
    }
}

fn direction_down(
    mut gstate: ResMut<GState>,
    mut data_panel: ResMut<DataPanel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Moveable, &mut Pos, &mut TransformBlock), With<Moveable>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    match gstate.0 {
        GameState::Pause | GameState::Idle | GameState::Stop => return,
        _ => {}
    }
    if (keyboard.just_pressed(KeyCode::ArrowDown)) {
        let effect = asset_server.load("sounds/plop.ogg");
        audio.set_volume(10.);
        audio.play(effect);

        for (mut data, mut pos, mut block) in query {
            let block_data = &mut block as &mut TransformBlock;
            let pos_data = &mut pos as &mut Pos;
            let collision = Collision {
                pos: pos_data.clone(),
            };
            if collision.is_exist_when_translate(block_data, Vec2::new(0., 1.)) {
                return;
            }

            if collision.is_exists_with_data_panel(&data_panel, block_data) {
                return;
            }

            pos.row += 1;
        }
    }
}

fn merge_data(
    mut gstate: ResMut<GState>,
    mut data_panel: ResMut<DataPanel>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Pos, &mut TransformBlock), With<Moveable>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    match gstate.0 {
        GameState::Pause | GameState::Idle | GameState::Stop => return,
        _ => {}
    }

    for (entity, mut pos, mut block) in query {
        let cur_pos = &mut pos as &mut Pos;
        let block_data = &mut block as &mut TransformBlock;
        let collision = Collision {
            pos: cur_pos.clone(),
        };
        let is_exist_when_translate =
            collision.is_exist_when_translate(block_data, Vec2::new(0., 1.));

        let new_pos = Pos::new(cur_pos.row + 1, cur_pos.col);
        let collision = Collision { pos: new_pos };
        let is_exists_with_data_panel =
            collision.is_exists_with_data_panel(&data_panel, block_data);

        let mut changed: Vec<i32> = vec![];
        if is_exist_when_translate || is_exists_with_data_panel {
            commands.entity(entity).despawn();

            let mut cnt = 0;
            for d in &block_data.blocks {
                if *d == 0 {
                    cnt += 1;
                    continue;
                }

                let row = cnt / block_data.cell_width;
                let col = cnt % block_data.cell_width;

                let idx_row = row + cur_pos.row;
                let idx_col = col + cur_pos.col;

                data_panel.blocks[idx_row as usize][idx_col as usize] = 1;
                changed.push(idx_row);
                cnt += 1;
            }
        }

        let rowRange = Range {
            start: 0,
            end: BLOCK_Y_COUNT as u32,
        };
        for row in rowRange {
            let mut is_full = true;
            for v in data_panel.blocks[row as usize] {
                if v == 0 {
                    is_full = false;
                    continue;
                }
            }

            if !is_full {
                continue;
            }

            let effect = asset_server.load("sounds/effect.ogg");
            audio.set_volume(10.);
            audio.play(effect);

            let mut krow = row;
            loop {
                if krow <= 0 {
                    break;
                }

                let kcolRange = Range {
                    start: 0,
                    end: BLOCK_X_COUNT as i32,
                };
                for kcol in kcolRange {
                    data_panel.blocks[krow as usize][kcol as usize] =
                        data_panel.blocks[krow as usize - 1][kcol as usize];
                }

                krow -= 1;
                if krow < 0 {
                    let kcolRange = Range {
                        start: 0,
                        end: BLOCK_X_COUNT as i32,
                    };
                    for kcol in kcolRange {
                        data_panel.blocks[0][kcol as usize] = 0;
                    }

                    break;
                }
            }
        }
    }
}
