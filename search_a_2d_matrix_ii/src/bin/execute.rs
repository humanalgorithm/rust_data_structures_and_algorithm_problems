use search_a_2d_matrix_ii::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("matrix: {:?}", data.matrix);
    println!("target: {:?}", data.target);
    let res = solution::search_matrix(data.matrix, data.target);
    println!("Result is {:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("matrix: {:?}", data.matrix);
    println!("target: {:?}", data.target);
    let res = solution::search_matrix(data.matrix, data.target);
    println!("Result is {:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
