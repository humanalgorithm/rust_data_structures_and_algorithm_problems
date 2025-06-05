#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicTacToe {
    board: Vec<Vec<i32>>,
    board_size: usize,
}

impl TicTacToe {
    pub fn new(n: i32) -> Self {
        let board = vec![vec![0; n as usize]; n as usize];

        return Self {
            board: board,
            board_size: n as usize,
        };
    }
    pub fn check_vertical(&self, col: usize, player: i32) -> bool {
        for x in 0..self.board_size {
            if self.board[x][col] != player {
                return false;
            }
        }
        return true;
    }
    pub fn check_horizontal(&self, row: usize, player: i32) -> bool {
        for x in 0..self.board_size {
            if self.board[row][x] != player {
                return false;
            }
        }
        return true;
    }
    pub fn check_diagonal_fw(&self, player: i32) -> bool {
        for x in 0..self.board_size {
            if self.board[x][x] != player {
                return false;
            }
        }
        return true;
    }
    pub fn check_diagonal_bw(&self, player: i32) -> bool {
        for x in (0..self.board_size).rev() {
            if self.board[x][self.board_size - 1 - x] != player {
                return false;
            }
        }
        return true;
    }

    pub fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let row = row as usize;
        let col = col as usize;
        self.board[row][col] = player;

        let vert = self.check_vertical(col, player);
        let hori = self.check_horizontal(row, player);
        let diag_1 = self.check_diagonal_fw(player);
        let diag_2 = self.check_diagonal_bw(player);
        if vert || hori || diag_1 || diag_2 {
            return player;
        }
        return 0;
    }
}
