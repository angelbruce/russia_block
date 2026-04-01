use bevy::prelude::*;
use std::collections::LinkedList;
use std::ops::Range;

pub const BG_COLOR_RED: f32 = 0.8;
pub const BG_COLOR_GREEN: f32 = 0.8;
pub const BG_COLOR_BLUE: f32 = 0.8;

pub const CELL_COLOR_RED: f32 = 0.0;
pub const CELL_COLOR_GREEN: f32 = 0.02;
pub const CELL_COLOR_BLUE: f32 = 0.0;
pub const BLOCK_DEF_COLOR_RED: f32 = 0.99;
pub const BLOCK_DEF_COLOR_GREEN: f32 = 0.99;
pub const BLOCK_DEF_COLOR_BLUE: f32 = 0.99;
pub const BLOCK_WIDTH: f32 = 600.0;
pub const BLOCK_HEIGHT: f32 = 1080.0;
pub const BLOCK_X_COUNT: f32 = 10.0;
pub const BLOCK_Y_COUNT: f32 = 18.0;
pub const BLOCK_PER_WIDTH: f32 = BLOCK_WIDTH / BLOCK_X_COUNT;
pub const BLOCK_PER_HEIGHT: f32 = BLOCK_HEIGHT / BLOCK_Y_COUNT;
pub const DEFAULT_POS_X: f32 = BLOCK_HEIGHT / 2.0;

#[derive(Component)]
pub struct Moveable;

#[derive(Component)]
pub struct DefaultCell;

#[derive(Component)]
pub struct Drawable;

#[derive(Component)]
pub struct Statable;

#[derive(Resource)]
pub struct FixedTime(pub Timer);

#[derive(Resource)]
pub struct DownTime(pub Timer);

#[derive(Resource)]
pub struct GState(pub GameState);

#[derive(Default, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Idle,
    Pause,
    Running,
    Stop,
}

/// cell which has a value to be 0 or 1 indicates valid or not.
#[derive(Component, Default)]
pub struct Cell(pub i32);
impl Cell {
    pub fn new() -> Cell {
        Cell(0)
    }
}

/// indicates row & col;
#[derive(Component, Default, Clone)]
pub struct Pos {
    pub row: i32,
    pub col: i32,
}
impl Pos {
    pub fn new(row: i32, col: i32) -> Pos {
        Pos { row, col }
    }

    pub fn default_born() -> Pos {
        Pos::new(0, BLOCK_X_COUNT as i32 / 2)
    }

    pub fn toWorld(&self) -> (f32, f32) {
        let x: f32 = -BLOCK_WIDTH / 2.0 + BLOCK_PER_WIDTH * self.col as f32 + BLOCK_PER_WIDTH / 2.0;
        let y: f32 =
            BLOCK_HEIGHT / 2.0 - BLOCK_PER_HEIGHT * self.row as f32 - BLOCK_PER_HEIGHT / 2.0;

        (x, y)
    }

    pub fn is_collision() {}
}

#[derive(Bundle)]
pub struct CellBundle {
    pub mesh2d: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub cell: Cell,
    pub pos: Pos,
}

pub struct BData {
    pub blocks: LinkedList<LinkedList<i8>>,
}

impl BData {
    pub fn new() -> BData {
        let mut blocks: LinkedList<LinkedList<i8>> = LinkedList::new();
        let rowRange = Range {
            start: 0,
            end: BLOCK_X_COUNT as i32,
        };
        for row in rowRange {
            let mut cols: LinkedList<i8> = LinkedList::new();
            let colRange = Range {
                start: 0,
                end: BLOCK_Y_COUNT as i32,
            };
            for col in colRange {
                cols.push_back(0);
            }
            blocks.push_back(cols);
        }

        BData { blocks }
    }
}

#[derive(Default)]
pub enum Direction {
    #[default]
    Up,
    Left,
    Down,
    Right,
}

#[derive(Default, Clone, Copy)]
pub enum BlockType {
    #[default]
    A,
    B,
    C,
    D,
}

const A_ALL_SIZE: [[i32; 2]; 4] = [[3, 2], [2, 3], [3, 2], [2, 3]];
const A_ALL_BLOCKS: [[i32; 6]; 4] = [
    [0, 1, 0, 1, 1, 1],
    [1, 0, 1, 1, 1, 0],
    [1, 1, 1, 0, 1, 0],
    [0, 1, 1, 1, 0, 1],
];

const B_ALL_SIZE: [[i32; 2]; 2] = [[4, 1], [1, 4]];
const B_ALL_BLOCKS: [[i32; 4]; 2] = [[1, 1, 1, 1], [1, 1, 1, 1]];

const C_ALL_SIZE: [[i32; 2]; 1] = [[2, 2]];
const C_ALL_BLOCKS: [[i32; 4]; 1] = [[1, 1, 1, 1]];

const D_ALL_BLOCKS: [[i32; 6]; 4] = [
    [1, 1, 1, 0, 1, 0],
    [1, 1, 1, 0, 0, 1],
    [0, 1, 0, 1, 1, 1],
    [1, 0, 0, 1, 1, 1],
];
const D_ALL_SIZE: [[i32; 2]; 4] = [[2, 3], [3, 2], [2, 3], [3, 2]];

#[derive(Component, Default, Clone)]
pub struct TransformBlock {
    pub blocks: Vec<i32>,
    pub typ: BlockType,
    pub seed: i32,
    pub cell_width: i32,
    pub cell_height: i32,
}

impl TransformBlock {
    pub fn new() -> TransformBlock {
        TransformBlock {
            blocks: vec![],
            typ: BlockType::A,
            ..default()
        }
    }

    pub fn rotate(&self) -> TransformBlock {
        return TransformBlock::create(self.typ, (self.seed + 1) % 4);
    }

    pub fn random() -> TransformBlock {
        let typ_idx = rand::random::<i32>() % 4;
        let seed = rand::random::<i32>() % 4;
        let typ = match typ_idx {
            0 => BlockType::A,
            1 => BlockType::B,
            2 => BlockType::C,
            3 => BlockType::D,
            _ => BlockType::A,
        };

        TransformBlock::create(typ, seed)
    }
    pub fn create(typ: BlockType, seed: i32) -> TransformBlock {
        let mut cell_width = 0;
        let mut cell_height = 0;
        let mut size: [i32; 2] = [0, 0];
        let mut blocks: Vec<i32> = vec![];
        match typ {
            BlockType::A => {
                let idx: usize = seed as usize % 4;
                let buff_blocks = A_ALL_BLOCKS[idx];
                size = A_ALL_SIZE[idx];
                for i in buff_blocks {
                    blocks.push(i);
                }
            }
            BlockType::B => {
                let idx: usize = seed as usize % 2;
                let buff_blocks = B_ALL_BLOCKS[idx];
                size = B_ALL_SIZE[idx];
                for i in buff_blocks {
                    blocks.push(i);
                }
            }
            BlockType::C => {
                let buff_blocks = C_ALL_BLOCKS[0];
                size = C_ALL_SIZE[0];
                for i in buff_blocks {
                    blocks.push(i);
                }
            }
            BlockType::D => {
                let idx: usize = seed as usize % 4;
                let buff_blocks = D_ALL_BLOCKS[idx];
                size = D_ALL_SIZE[idx];
                for i in buff_blocks {
                    blocks.push(i);
                }
            }
        }

        (cell_width, cell_height) = (size[0], size[1]);

        // let pos = Pos::new(BLOCK_X_COUNT as i32 / 2, 0);
        TransformBlock {
            blocks,
            typ,
            seed,
            cell_width,
            cell_height,
        }
    }
}

pub struct Collision {
    pub pos: Pos,
}
impl Collision {
    pub fn is_exist_when_transform(&self, transform_block: &TransformBlock) -> bool {
        let typ = transform_block.typ;
        let seed = transform_block.seed + 1 % 4;

        let new_block = TransformBlock::create(typ, seed);
        if (self.pos.col + new_block.cell_width) > (BLOCK_X_COUNT as i32) {
            true
        } else if (self.pos.row + new_block.cell_height) > (BLOCK_Y_COUNT as i32) {
            true
        } else {
            false
        }
    }

    pub fn is_exist_when_translate(&self, transform_block: &TransformBlock, dir: Vec2) -> bool {
        if (self.pos.row as f32 + dir.y < 0.0) {
            true
        } else if self.pos.col as f32 + dir.x < 0.0 {
            true
        } else if (self.pos.row + dir.y as i32 + (transform_block.cell_height))
            > (BLOCK_Y_COUNT as i32)
        {
            true
        } else if (self.pos.col + dir.x as i32 + (transform_block.cell_width))
            > (BLOCK_X_COUNT as i32)
        {
            true
        } else {
            false
        }
    }

    pub fn is_exists_with_data_panel(
        &self,
        data_panel: &DataPanel,
        transform_block: &TransformBlock,
    ) -> bool {
        if self.pos.row > (BLOCK_Y_COUNT as i32 - 1) {
            return true;
        }

        if self.pos.col > (BLOCK_X_COUNT as i32 - 1) {
            return true;
        }

        let mut cnt = 0;
        for b in &transform_block.blocks {
            if *b == 0 {
                cnt += 1;
                continue;
            }

            let row = cnt / transform_block.cell_width;
            let col = cnt % transform_block.cell_width;

            let x = self.pos.row + row;
            let y = self.pos.col + col;

            if x > (BLOCK_Y_COUNT as i32 - 1) {
                return true;
            }

            if y > (BLOCK_X_COUNT as i32 - 1) {
                return true;
            }

            let val = data_panel.blocks[x as usize][y as usize];
            // println!(" value is : {}:{}= {}", x, y, val);
            if val == 1 {
                return true;
            }

            cnt += 1;
        }

        return false;
    }
}

#[derive(Resource, Default, Clone, Copy)]
pub struct DataPanel {
    pub blocks: [[u32; BLOCK_X_COUNT as usize]; BLOCK_Y_COUNT as usize],
}

impl DataPanel {
    pub fn new() -> DataPanel {
        DataPanel { ..default() }
    }
}
