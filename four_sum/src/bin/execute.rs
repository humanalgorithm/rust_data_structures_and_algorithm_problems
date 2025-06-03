use four_sum::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("target: {:?}", data.target);
    let res = solution::four_sum(data.nums, data.target);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("target: {:?}", data.target);
    let res = solution::four_sum(data.nums, data.target);
    println!("\nResult: \n{:?}", res);
}
fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
