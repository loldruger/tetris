mod object;
mod renderer;
use object::{Board, Block};

use std::{thread, time};

fn main() {
    let mut board = Board::new(20, 10);
    let block_t = Block::new(
        (0, 0), [
            [0, 1, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ], 
        (0, 0, 0));
    
    board.spawn(block_t);

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
