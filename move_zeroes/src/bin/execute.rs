use move_zeroes::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::move_zeroes(&mut data.nums);
    println!("Result is {:?}\n", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::move_zeroes(&mut data.nums);
    println!("Result is {:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
