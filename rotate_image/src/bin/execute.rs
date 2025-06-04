use rotate_image::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("matrix: {:?}", data.matrix);
    let res = solution::rotate(&mut data.matrix);
    println!("Result is {:?}", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("matrix: {:?}", data.matrix);
    let res = solution::rotate(&mut data.matrix);
    println!("Result is {:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
