pub struct Block {
    shape: [[u8; 4]; 4],
    color: (u8, u8, u8)
}

impl Block {
    pub fn new(shape: [[u8; 4]; 4], color: (u8, u8, u8)) -> Self {
        Block {shape, color}
    }

    pub fn get_shape(&self) -> [[u8; 4]; 4] {
        self.shape
    }

    pub fn rotate(&mut self, is_clockwise: bool) {
        let mut block_rotated = [
            [b'.', b'.', b'.', b'.'],
            [b'.', b'.', b'.', b'.'],
            [b'.', b'.', b'.', b'.'],
            [b'.', b'.', b'.', b'.']
        ];

        for i in 0..4 {
            for j in 0..4 {
                if is_clockwise {
                    block_rotated[i][j] = self.shape[3-j][i]
                } else {
                    block_rotated[i][j] = self.shape[j][3-i]
                }
            }
        }

        self.shape = block_rotated;
    }
}