use palindromic_substrings::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input: ");
    println!("s: {:?}", data.s);
    let res = solution::count_substrings(data.s);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("\nInput: ");
    println!("s: {:?}", data.s);
    let res = solution::count_substrings(data.s);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("\nRunning Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
