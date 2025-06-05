use reverse_string::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("s is {:?}", data.s);
    let res = solution::reverse_string(&mut data.s);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("s is {:?}", data.s);
    let res = solution::reverse_string(&mut data.s);
    println!("\nResult: \n{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
