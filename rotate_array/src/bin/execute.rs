use rotate_array::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("k: {:?}", data.k);
    let res = solution::rotate(&mut data.nums, data.k);
    println!("\nResult: \n{:?}\n", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    println!("k: {:?}", data.k);
    let res = solution::rotate(&mut data.nums, data.k);
    println!("\nResult: \n{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 1...");
    case_2();
}
