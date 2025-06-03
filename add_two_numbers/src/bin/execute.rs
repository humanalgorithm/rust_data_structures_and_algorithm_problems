use add_two_numbers::{data, solution};

fn case_1() {
    let l1 = data::data_1_l1();
    let l2 = data::data_1_l2();
    println!("Input data:");
    println!("l1 is {:?}", l1);
    println!("l2 is {:?}", l2);
    let res = solution::add_two_numbers(l1, l2);
    println!("\nResult: \n {:?}", res);
}

fn case_2() {
    let l1 = data::data_2_empty();
    let l2 = data::data_2_empty();
    println!("Input data:");
    println!("l1 is {:?}", l1);
    println!("l2 is {:?}", l2);
    let res = solution::add_two_numbers(l1, l2);
    println!("\nResult: \n {:?}", res);
}

fn case_3() {
    let l1 = data::data_3_l1();
    let l2 = data::data_3_l2();
    println!("Input data:");
    println!("l1 is {:?}", l1);
    println!("l2 is {:?}", l2);
    let res = solution::add_two_numbers(l1, l2);
    println!("\nResult: \n {:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    println!("\nRunning Case 3...");
    case_3();
}
