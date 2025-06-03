use search_in_rotated_sorted_array::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("target: {:?}", data.target);
    let res = solution::search(data.nums, data.target);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("target: {:?}", data.target);
    let res = solution::search(data.nums, data.target);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("target: {:?}", data.target);
    let res = solution::search(data.nums, data.target);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    println!("\nRunning Case 3...");
    case_3();
}
