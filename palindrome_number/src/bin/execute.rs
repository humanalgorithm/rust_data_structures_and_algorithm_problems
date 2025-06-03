use palindrome_number::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("x is {:?}", data.x);
    let res = solution::is_palindrome(data.x);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("x is {:?}", data.x);
    let res = solution::is_palindrome(data.x);
    println!("\nResult: \n{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("x is {:?}", data.x);
    let res = solution::is_palindrome(data.x);
    println!("\nResult: \n{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    println!("\nRunning Case 3...");
    case_3();
}
