use combination_sum_ii::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("candidates: {:?}", data.candidates);
    println!("target: {:?}", data.target);
    let res = solution::combination_sum2(data.candidates, data.target);
    println!("Result is {:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("candidates: {:?}", data.candidates);
    println!("target: {:?}", data.target);
    let res = solution::combination_sum2(data.candidates, data.target);
    println!("Result is {:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
