use remove_duplicates_from_sorted_array::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::remove_duplicates(&mut data.nums);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("nums: {:?}", data.nums);
    let res = solution::remove_duplicates(&mut data.nums);
    println!("\nResult: \n{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
