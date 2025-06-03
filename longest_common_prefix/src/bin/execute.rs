use longest_common_prefix::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("strs is {:?}", data.strs);
    let res = solution::longest_common_prefix(data.strs);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("strs is {:?}", data.strs);
    let res = solution::longest_common_prefix(data.strs);
    println!("\nResult: \n{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
