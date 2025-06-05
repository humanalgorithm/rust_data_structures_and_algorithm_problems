use top_k_frequent_elements::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("k: {:?}", data.k);
    let res = solution::top_k_frequent(data.nums, data.k);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("k: {:?}", data.k);
    let res = solution::top_k_frequent(data.nums, data.k);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
