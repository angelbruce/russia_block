use crate::component::*;
use bevy::prelude::*;
use std::ops::Range;

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime(Timer::from_seconds(0.02, TimerMode::Repeating)));

        app.add_systems(Update, draw);
    }
}

fn draw(
    time: Res<Time>,
    mut fixed_time: ResMut<FixedTime>,
    mut data_panel: ResMut<DataPanel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Drawable)>,
    queryTransform: Query<(&Moveable, &Pos, &TransformBlock)>,
) {
    if !fixed_time.0.tick(time.delta()).just_finished() {
        return;
    }

    for (entity, _) in query {
        commands.entity(entity).despawn();
    }

    for (_, pos, transform_block) in queryTransform {
        let block = &transform_block as &TransformBlock;
        let pos_data = &pos as &Pos;

        let mut cnt = 0;
        for k in &block.blocks {
            if *k == 0 {
                cnt += 1;
                continue;
            }

            let row = cnt / block.cell_width + pos_data.row;
            let col = cnt % block.cell_width + pos_data.col;
            cnt += 1;

            let pos = Pos::new(row, col);
            let (x, y) = pos.toWorld();

            let rect = Rectangle::new(BLOCK_PER_WIDTH - 10 as f32, BLOCK_PER_HEIGHT - 10 as f32);
            let shape = meshes.add(rect);
            let color = Color::srgba(CELL_COLOR_RED, CELL_COLOR_GREEN, CELL_COLOR_BLUE, 1.0);
            let material = MeshMaterial2d(materials.add(color));

            commands.spawn((
                Mesh2d(shape),
                material,
                Transform::from_xyz(x, y, 0.0),
                Drawable,
            ));
        }
    }

    let rowRange = Range {
        start: 0,
        end: BLOCK_Y_COUNT as i32,
    };

    for row in rowRange {
        let colRange = Range {
            start: 0,
            end: BLOCK_X_COUNT as i32,
        };
        for col in colRange {
            if data_panel.blocks[row as usize][col as usize] == 0 {
                continue;
            }

            let pos = Pos::new(row, col);
            let (x, y) = pos.toWorld();

            let rect = Rectangle::new(BLOCK_PER_WIDTH - 10 as f32, BLOCK_PER_HEIGHT - 10 as f32);
            let shape = meshes.add(rect);
            let color = Color::srgba(CELL_COLOR_RED, CELL_COLOR_GREEN, CELL_COLOR_BLUE, 1.0);
            let material = MeshMaterial2d(materials.add(color));

            commands.spawn((
                Mesh2d(shape),
                material,
                Transform::from_xyz(x, y, 0.0),
                Drawable,
            ));
        }
    }
}
