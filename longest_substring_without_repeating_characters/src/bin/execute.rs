use longest_substring_without_repeating_characters::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input: ");
    println!("s is {:?}", data.s);
    let res = solution::length_of_longest_substring(data.s);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input:");
    println!("s is {:?}", data.s);
    let res = solution::length_of_longest_substring(data.s);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input:");
    println!("s is {:?}", data.s);
    let res = solution::length_of_longest_substring(data.s);
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
