use valid_sudoku::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("board: {:?}", data.board);
    let res = solution::is_valid_sudoku(data.board);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("board: {:?}", data.board);
    let res = solution::is_valid_sudoku(data.board);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
