use divide_two_integers::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("dividend: {:?}", data.dividend);
    println!("divisor: {:?}", data.divisor);
    let res = solution::divide(data.dividend, data.divisor);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("dividend: {:?}", data.dividend);
    println!("divisor: {:?}", data.divisor);
    let res = solution::divide(data.dividend, data.divisor);
    println!("\nResult: \n{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
