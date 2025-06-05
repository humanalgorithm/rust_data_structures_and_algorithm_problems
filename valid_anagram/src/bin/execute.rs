use valid_anagram::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("s is {:?}", data.s);
    println!("t is {:?}", data.t);
    let res = solution::is_anagram(data.s, data.t);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("s is {:?}", data.s);
    println!("t is {:?}", data.t);
    let res = solution::is_anagram(data.s, data.t);
    println!("\nResult: \n{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
