mod board;
use board::Board;

fn main() {
    let mut board = Board::new();
    backtracking_queens(&mut board);
    board.print();
}

fn backtracking_queens(board: &mut Board) -> bool{
    for row in 0..9 {
       for col in 0..9 {
           if board.get(row, col) == Some(0){
               for value in 1..=9{
                  if board.set(row, col, value){
                      if backtracking_queens(board){
                          return true
                      }
                      board.unset(row, col);
                  } 
               }
               return false
           }
       } 
    }
    return true
}
