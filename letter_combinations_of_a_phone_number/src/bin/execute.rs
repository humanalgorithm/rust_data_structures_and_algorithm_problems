use letter_combinations_of_a_phone_number::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input:");
    println!("digits: {:?}", data.digits);
    let res = solution::letter_combinations(data.digits);
    println!("Result:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input:");
    println!("digits: {:?}", data.digits);
    let res = solution::letter_combinations(data.digits);
    println!("Result:");
    println!("{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input:");
    println!("digits: {:?}", data.digits);
    let res = solution::letter_combinations(data.digits);
    println!("Result:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    println!("\nRunning Case 2...");
    case_3();
}
