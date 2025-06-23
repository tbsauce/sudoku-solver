mod board;
use board::Board;

fn main() {
    let mut board = Board::new();
    board.print();
}

// fn backtracking_queens(&mut board, u8 row, u8, col) -> bool{
//     if row == 9{
//        true 
//     }
//
//     if (backtracking_queens(board, row)){
//
//     }
// }
