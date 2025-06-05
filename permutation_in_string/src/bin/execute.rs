use permutation_in_string::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("\nInput: ");
    println!("s1: {:?}", data.s1);
    println!("s1: {:?}", data.s2);
    let res = solution::check_inclusion(data.s1, data.s2);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("\nInput: ");
    println!("s1: {:?}", data.s1);
    println!("s2: {:?}", data.s2);
    let res = solution::check_inclusion(data.s1, data.s2);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
