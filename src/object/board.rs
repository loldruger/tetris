use super::{Block, block::Placable};

pub struct Board {
    width: usize,
    height: usize,
    data: Vec<Vec<u8>>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = Self {
            width,
            height,
            data: Vec::new()
        };

        board.flush();

        return board;
    }

    pub fn update(&mut self) {
        self.flush();

        // for block in &self.data {
        //     for i in 0..4 {
        //         for j in 0..4 {
        //             self.data[i][j] = block.get_shape()[i][j];
        //         }
        //     }
        // }

        // io::stdout().write_all(
        //     &self.buffer
        //         .clone()
        //         .into_iter()
        //         .flatten()
        //         .collect::<Vec<u8>>())?;

    }

    pub fn spawn<const N: usize>(&mut self, block: impl Placable<N>) {
        let a = block.get_shape();
        let pos = block.get_position();
        let size = block.get_shape().to_vec().len();

        for i in 0..size {
            for j in 0..size {
                self.data[pos.0 + j][pos.1 + i] = a[j][i];
            }
        }
    }

    fn flush(&mut self) {
        // io::stdout().flush()?;
        // io::stdout().write_all(b"\x1B[2J\x1B[1;1H")?;
        self.data = vec![vec![0; self.width]; self.height];

        // for i in 0..self.height {
        //     self.data[i][self.width-1] = b'\n';
        // }
    }

    pub fn display(&self) {
        // let a = self.data
        //     .clone()
        //     .into_iter()
        //     .flatten()
        //     .collect::<Vec<u8>>();

        for i in 0..self.height {
            print!("\n");
            for j in 0..self.width {
                print!("{}", self.data[i][j]);
            }
        }
    }
}