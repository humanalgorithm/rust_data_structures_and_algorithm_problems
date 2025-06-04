use insert_interval::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("intervals: {:?}", data.intervals);
    println!("new_interval: {:?}", data.new_interval);
    let res = solution::insert(data.intervals, data.new_interval);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("intervals: {:?}", data.intervals);
    println!("new_interval: {:?}", data.new_interval);
    let res = solution::insert(data.intervals, data.new_interval);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
