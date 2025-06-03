use search_insert_position::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("target: {:?}", data.target);
    let res = solution::search_insert(data.nums, data.target);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("target: {:?}", data.target);
    let res = solution::search_insert(data.nums, data.target);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("target: {:?}", data.target);
    let res = solution::search_insert(data.nums, data.target);
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
