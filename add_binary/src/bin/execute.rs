use add_binary::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("a: {:?}", data.a);
    println!("b: {:?}", data.b);
    let res = solution::add_binary(data.a, data.b);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("a: {:?}", data.a);
    println!("b: {:?}", data.b);
    let res = solution::add_binary(data.a, data.b);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
