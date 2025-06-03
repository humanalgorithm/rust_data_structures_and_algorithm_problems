use remove_element::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("val: {:?}", data.val);
    let res = solution::remove_element(&mut data.nums, data.val);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("val: {:?}", data.val);
    let res = solution::remove_element(&mut data.nums, data.val);
    println!("\nResult: \n{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    println!("\nRunning Case 3...");
}
