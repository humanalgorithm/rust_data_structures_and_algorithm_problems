use find_duplicate_file_in_system::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("\nInput: ");
    println!("paths: {:?}", data.paths);
    let res = solution::find_duplicate(data.paths);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("\nInput: ");
    println!("paths: {:?}", data.paths);
    let res = solution::find_duplicate(data.paths);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
