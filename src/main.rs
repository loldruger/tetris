mod object;
mod renderer;
use object::{Board, Block, constant, block::Placable};

use std::{thread, time};

fn main() {
    let mut board = Board::new(10, 20);
    let block_o = Block::new(
        (0, 0),
        constant::BLOCK_O, 
        (0, 0, 0)
    );

    let block_t = Block::new(
        (2, 0), 
        constant::BLOCK_T,
        (0, 0, 0)
    );
    
    let block_i = Block::new(
        (3, 3),
        constant::BLOCK_I,
        (0, 0, 0)
    );
    
    board.spawn(block_o);
    board.spawn(block_t);
    board.spawn(block_i);
    // board.update();
    
    loop {
        if let Some(a) = board.get_current_block() {
            a.rotate(true);
        }

        board.update();
        board.display();

        let ms = time::Duration::from_millis(1000);
        let now = time::Instant::now();

        thread::sleep(ms);
    }
}
