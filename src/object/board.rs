use super::Block;

pub struct Board {
    width: usize,
    height: usize,
    blocks: Vec<Block>,
    data: Vec<Vec<u8>>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = Board {
            width,
            height,
            blocks: Vec::new(),
            data: Vec::new()
        };

        board.flush();

        return board;
    }

    pub fn update(&mut self) {
        self.flush();

        for block in &self.blocks {
            for i in 0..4 {
                for j in 0..4 {
                    self.data[i][j] = block.get_shape()[i][j];
                }
            }
        }

        // io::stdout().write_all(
        //     &self.buffer
        //         .clone()
        //         .into_iter()
        //         .flatten()
        //         .collect::<Vec<u8>>())?;

    }

    pub fn spawn(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_current_block(&mut self) -> Option<&mut Block> {
        self.blocks.last_mut()
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
        let a = self.data
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>();

        print!("{:?}", &a);
    }
}