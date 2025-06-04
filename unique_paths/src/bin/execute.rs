use unique_paths::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("m: {:?}", data.m);
    println!("n: {:?}", data.n);
    let res = solution::unique_paths(data.m, data.n);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("m: {:?}", data.m);
    println!("n: {:?}", data.n);
    let res = solution::unique_paths(data.m, data.n);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
