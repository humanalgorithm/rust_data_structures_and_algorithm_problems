use subarray_sum_equals_k::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("\nInput data:");
    println!("nums: {:?}", data.nums);
    println!("k: {:?}", data.k);
    let res = solution::subarray_sum(data.nums, data.k);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("\nInput data:");
    println!("nums: {:?}", data.nums);
    println!("k: {:?}", data.k);
    let res = solution::subarray_sum(data.nums, data.k);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
