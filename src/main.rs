mod object;
mod renderer;
use object::{Board, Block};

use std::{thread, time};

fn main() {
    let mut board = Board::new(10, 20);
    let block_l = Block::new(
        (0, 0), [
            [0, 0, 0, 0],
            [1, 1, 1, 1],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ], 
        (0, 0, 0));

    let block_t = Block::new(
        (2, 0), [
            [0, 1, 0],
            [1, 1, 1],
            [0, 0, 0],
        ], 
        (0, 0, 0));
    
    let block_z = Block::new(
        (3, 3), [
            [0, 1, 1],
            [1, 1, 0],
            [0, 0, 0],
        ], 
        (0, 0, 0));
    
    board.spawn(block_l);
    board.spawn(block_t);
    board.spawn(block_z);
    // board.update();
    board.display();
    
    // loop {
    //     if let Some(a) = board.get_current_block() {
    //         a.rotate(true);
    //     }

    //     board.update();
    //     board.display();

    //     let ms = time::Duration::from_millis(1000);
    //     let now = time::Instant::now();

    //     thread::sleep(ms);
    // }
}
