pub trait Placable<const N: usize, const M: usize> {
    fn get_shape(&self) -> [[u8; N]; M];
    fn get_position(&self) -> (usize, usize);
    fn set_position(&mut self, pos: (usize, usize));

    fn rotate(&mut self, is_clockwise: bool);
}

pub struct Block<const N: usize, const M: usize> {
    position: (usize, usize),
    shape: [[u8; N]; M],
    color: (u8, u8, u8) //RGB
}

impl<const N: usize, const M: usize> Placable<N, M> for Block<N, M> {
    fn get_shape(&self) -> [[u8; N]; M] {
        self.shape
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, pos: (usize, usize)) {
        self.position = pos;
    }

    fn rotate(&mut self, is_clockwise: bool) {
        let mut block_rotated = [[0; N]; M];

        for i in 0..N {
            for j in 0..M {
                if is_clockwise {
                    block_rotated[i][j] = self.shape[N-1-j][i]
                } else {
                    block_rotated[i][j] = self.shape[j][M-1-i]
                }
            }
        }

        self.shape = block_rotated;
    }
}

impl<const N: usize, const M: usize> Block<N, M> {
    pub fn new(position: (usize, usize), shape: [[u8; N]; M], color: (u8, u8, u8)) -> Self {
        Self {position, shape, color}
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }
}