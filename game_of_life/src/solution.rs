pub fn game_of_life(board: &mut Vec<Vec<i32>>) -> &mut Vec<Vec<i32>> {
    let board_2 = board.clone();

    for i in 0..board_2.len() {
        for j in 0..board_2[0].len() {
            let ul = if i > 0 && j > 0 {
                board_2[i - 1][j - 1]
            } else {
                0
            };
            let um = if i > 0 { board_2[i - 1][j] } else { 0 };
            let ur = if i > 0 && j + 1 < board_2[i].len() {
                board_2[i - 1][j + 1]
            } else {
                0
            };

            let dl = if i + 1 < board_2.len() && j > 0 {
                board_2[i + 1][j - 1]
            } else {
                0
            };
            let dm = if i + 1 < board_2.len() {
                board_2[i + 1][j]
            } else {
                0
            };
            let dr = if i + 1 < board_2.len() && j + 1 < board_2[i].len() {
                board_2[i + 1][j + 1]
            } else {
                0
            };

            let lm = if j > 0 { board_2[i][j - 1] } else { 0 };
            let rm = if j + 1 < board_2[i].len() {
                board_2[i][j + 1]
            } else {
                0
            };

            let neighbor_count = ul + um + ur + dl + dm + dr + lm + rm;

            // live checks
            if board_2[i][j] == 1 {
                if neighbor_count < 2 {
                    board[i][j] = 0;
                } else if neighbor_count == 2 || neighbor_count == 3 {
                    board[i][j] = 1;
                } else if neighbor_count > 3 {
                    board[i][j] = 0;
                }
            }
            // dead check
            else {
                if neighbor_count == 3 {
                    board[i][j] = 1;
                }
            }
        }
    }
    board
}
