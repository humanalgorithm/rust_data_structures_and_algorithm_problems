use house_robber::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::rob(data.nums);
    println!("\nResult: \n{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::rob(data.nums);
    println!("\nResult: \n{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 1...");
    case_2();
}
