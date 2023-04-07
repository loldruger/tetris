pub trait Placable<const N: usize> {
    fn get_shape(&self) -> [[u8; N]; N];
    fn get_position(&self) -> (usize, usize);
    fn set_position(&mut self, pos: (usize, usize));

    fn rotate(&mut self, is_clockwise: bool);
}

pub struct Block<const N: usize> {
    position: (usize, usize),
    shape: [[u8; N]; N],
    color: (u8, u8, u8) //RGB
}

impl<const N: usize> Placable<N> for Block<N> {
    fn get_shape(&self) -> [[u8; N]; N] {
        self.shape
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, pos: (usize, usize)) {
        self.position = pos;
    }

    fn rotate(&mut self, is_clockwise: bool) {
        let mut block_rotated = [[0; N]; N];

        for i in 0..N {
            for j in 0..N {
                if is_clockwise {
                    block_rotated[i][j] = self.shape[N-1-j][i]
                } else {
                    block_rotated[i][j] = self.shape[j][N-1-i]
                }
            }
        }

        self.shape = block_rotated;
    }
}

impl<const N: usize> Block<N> {
    pub fn new(position: (usize, usize), shape: [[u8; N]; N], color: (u8, u8, u8)) -> Self {
        Self {position, shape, color}
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }
}