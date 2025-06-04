use minimum_path_sum::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("grid: {:?}", data.grid);
    let res = solution::min_path_sum(data.grid);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("grid: {:?}", data.grid);
    let res = solution::min_path_sum(data.grid);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
