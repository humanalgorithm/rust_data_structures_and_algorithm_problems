use merge_intervals::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("intervals: {:?}", data.intervals);
    let res = solution::merge(data.intervals);
    println!("Result is {:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("intervals: {:?}", data.intervals);
    let res = solution::merge(data.intervals);
    println!("Result is {:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
