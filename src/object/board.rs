use std::io::{self, Write};
use super::Block;
use super::super::renderer::{Renderer};

pub struct Board {
    width: usize,
    height: usize,
    blocks: Vec<Block>,
    buffer: Vec<Vec<u8>>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = Board {
            width,
            height,
            blocks: Vec::new(),
            buffer: Vec::new()
        };

        board.flush().ok();

        return board;
    }

    pub fn render(&mut self) -> io::Result<()> {
        self.flush()?;

        for block in &self.blocks {
            for i in 0..4 {
                for j in 0..4 {
                    self.buffer[i][j] = block.get_shape()[i][j];
                    self.buffer[i][j+1] = block.get_shape()[i][j];
                    self.buffer[i][j+2] = block.get_shape()[i][j];
                    self.buffer[i][j+3] = block.get_shape()[i][j];
                }
            }
        }

        io::stdout().write_all(
            &self.buffer
                .clone()
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>())?;

        Ok(())
    }

    pub fn spawn(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_current_block(&mut self) -> Option<&mut Block> {
        self.blocks.last_mut()
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()?;
        io::stdout().write_all(b"\x1B[2J\x1B[1;1H")?;
        self.buffer = vec![vec![b'.'; self.width]; self.height];

        for i in 0..self.height {
            self.buffer[i][self.width-1] = b'\n';
        }

        Ok(())
    }
}