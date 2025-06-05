use game_of_life::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("board: {:?}", data.board);
    let res = solution::game_of_life(&mut data.board);
    println!("Result is {:?}\n", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("board: {:?}", data.board);
    let res = solution::game_of_life(&mut data.board);
    println!("Result is {:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
