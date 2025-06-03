use next_permutation::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::next_permutation(&mut data.nums);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::next_permutation(&mut data.nums);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_3() {
    let mut data = data::data_3();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::next_permutation(&mut data.nums);
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
