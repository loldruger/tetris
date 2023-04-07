use std::io::{self, Write};

use super::{Block, block::Placable};

pub struct Board {
    width: usize,
    height: usize,
    block: Option<Block<4, 4>>,
    data: Vec<Vec<u8>>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = Self {
            width,
            height,
            block: None,
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

    pub fn get_current_block<'a>(&'a mut self) -> &'a mut Option<Block<4, 4>> {
        &mut self.block
    }

    pub fn spawn<const N: usize, const M: usize>(&mut self, block: impl Placable<N, M>) {
        let shape = block.get_shape();
        let pos = block.get_position();
        let height = shape.len();
        let width = shape[0].len();

        for i in 0..width {
            for j in 0..height {
                if shape[j][i] > 0 {
                    self.data[pos.0 + j][pos.1 + i] = shape[j][i];
                }
            }
        }

        self.block = Some(block);
    }

    fn flush(&mut self) {
        let data_temp = self.data.clone();
        
        io::stdout().flush().unwrap();
        io::stdout().write_all(b"\x1B[2J\x1B[1;1H").unwrap();

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