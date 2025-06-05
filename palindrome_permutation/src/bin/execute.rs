use palindrome_permutation::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("s is {:?}", data.s);
    let res = solution::can_permute_palindrome(data.s);
    println!("\nResult: \n{:?}", res);
}
fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("s is {:?}", data.s);
    let res = solution::can_permute_palindrome(data.s);
    println!("\nResult: \n{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("s is {:?}", data.s);
    let res = solution::can_permute_palindrome(data.s);
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
