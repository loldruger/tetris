pub const MAXIMUM_BLOCK_WIDTH: usize = 4;
pub const MAXIMUM_BLOCK_HEIGHT: usize = 4;

pub const BLOCK_I: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [0, 0, 0, 0],
    [0, 0, 0, 0]
];

pub const BLOCK_J: [[u8; 3]; 3] = [
    [1, 0, 0],
    [1, 1, 1],
    [0, 0, 0],
];

pub const BLOCK_L: [[u8; 3]; 3] = [
    [0, 0, 1],
    [1, 1, 1],
    [0, 0, 0],
];

pub const BLOCK_O: [[u8; 4]; 3] = [
    [0, 1, 1, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];

pub const BLOCK_S: [[u8; 3]; 3] = [
    [0, 1, 1],
    [1, 1, 0],
    [0, 0, 0],
];

pub const BLOCK_T: [[u8; 3]; 3] = [
    [0, 1, 0],
    [1, 1, 1],
    [0, 0, 0],
];

pub const BLOCK_Z: [[u8; 3]; 3] = [
    [1, 1, 0],
    [0, 1, 1],
    [0, 0, 0],
];