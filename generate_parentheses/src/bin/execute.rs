use generate_parentheses::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("n: {:?}", data.n);
    let res = solution::generate_parenthesis(data.n);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("n: {:?}", data.n);
    let res = solution::generate_parenthesis(data.n);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    println!("\nRunning Case 3...");
}
