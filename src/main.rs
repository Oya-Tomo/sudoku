use sudoku::put_nums;

use crate::sudoku::get_putable_nums;

mod sudoku;

fn main() {
    let mut board = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 0, 0],
        vec![6, 0, 4, 0, 0, 8, 7, 0, 0],
        vec![0, 3, 0, 6, 0, 9, 0, 8, 0],
        vec![0, 0, 8, 0, 0, 0, 4, 0, 1],
        vec![0, 5, 0, 0, 0, 0, 6, 0, 0],
        vec![0, 0, 0, 0, 5, 0, 0, 2, 0],
        vec![0, 0, 7, 0, 8, 6, 9, 0, 0],
        vec![0, 6, 0, 0, 3, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 4, 1, 0, 8],
    ];

    for _ in 0..100 {
        board = put_nums(&board, get_putable_nums(&board));
    }
    println!("{:?}", board);
}
