use std::collections::HashSet;

pub fn print_board(board: &Vec<Vec<char>>) {
    for i in 0..board.len() {
        let mut print_str = "".to_string();
        for j in 0..board[i].len() {
            let char_print: String = format!("{}", board[i][j].to_string());
            if (j + 1) % 3 == 0 {
                let concat: &str = "  |  ";
                print_str = print_str + char_print.as_str() + ", " + concat;
            } else {
                print_str = print_str + char_print.as_str() + ", ";
            }
        }
        println!("{:?}", print_str);
        if (i + 1) % 3 == 0 {
            println!("");
        }
    }
}
pub fn check_rows(board: &Vec<Vec<char>>, row_len: usize, col_len: usize) -> bool {
    for i in 0..row_len {
        let mut row_chars: HashSet<char> = HashSet::new();
        for j in 0..col_len {
            let this_char = board[i][j];
            match row_chars.get(&this_char) {
                Some(_) => return false,
                None => {}
            }
            if this_char.is_alphanumeric() {
                row_chars.insert(this_char);
            }
        }
    }
    return true;
}
pub fn check_cols(board: &Vec<Vec<char>>, row_len: usize, col_len: usize) -> bool {
    for i in 0..col_len {
        let mut col_chars: HashSet<char> = HashSet::new();
        for j in 0..row_len {
            let this_char = board[j][i];
            match col_chars.get(&this_char) {
                Some(_) => return false,
                None => {}
            }
            if this_char.is_alphanumeric() {
                col_chars.insert(this_char);
            }
        }
    }
    return true;
}
pub fn check_grid(board: &Vec<Vec<char>>, row_len: usize, col_len: usize) -> bool {
    let mut box_row = 0;

    while (box_row * 3) + 3 < row_len + 1 {
        let mut box_col = 0;
        while (box_col * 3) + 3 < col_len + 1 {
            let mut box_chars: HashSet<char> = HashSet::new();
            for i in box_row * 3..box_row * 3 + 3 {
                for j in box_col * 3..box_col * 3 + 3 {
                    let this_char = board[i][j];
                    match box_chars.get(&this_char) {
                        Some(_) => return false,
                        None => {}
                    }
                    if this_char.is_alphanumeric() {
                        box_chars.insert(this_char);
                    }
                }
            }
            box_col += 1;
        }
        box_row += 1;
    }
    return true;
}
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let (row_len, col_len) = (board.len(), board[0].len());
    let rows_valid = check_rows(&board, row_len, col_len);
    let cols_valid = check_cols(&board, row_len, col_len);
    let grid_valid = check_grid(&board, row_len, col_len);
    return rows_valid && cols_valid && grid_valid;
}
