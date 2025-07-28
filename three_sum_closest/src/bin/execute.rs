use three_sum_closest::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input:");
    println!("nums: {:?}, target: {:?}", data.nums, data.target);
    let res = solution::three_sum_closest(data.nums, data.target);
    println!("Result:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input:");
    println!("nums: {:?}, target: {:?}", data.nums, data.target);
    let res = solution::three_sum_closest(data.nums, data.target);
    println!("Result:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
