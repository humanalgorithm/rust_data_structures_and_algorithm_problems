use unique_binary_search_trees::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("n: {:?}", data.n);
    let res = solution::num_trees(data.n);
    println!("\nResult:");
    println!("{:?} \n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("n: {:?}", data.n);
    let res = solution::num_trees(data.n);
    println!("\nResult:");
    println!("{:?} \n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
