use valid_parentheses::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("s: {:?}", data.s);
    let res = solution::is_valid(data.s);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("s: {:?}", data.s);
    let res = solution::is_valid(data.s);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("s: {:?}", data.s);
    let res = solution::is_valid(data.s);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_4() {
    let data = data::data_4();
    println!("Input data:");
    println!("s: {:?}", data.s);
    let res = solution::is_valid(data.s);
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
    println!("\nRunning Case 4...");
    case_4();
}
