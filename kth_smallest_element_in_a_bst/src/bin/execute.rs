use kth_smallest_element_in_a_bst::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("root: {:?}", data.root);
    let res = solution::kth_smallest(data.root, data.k);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("root: {:?}", data.root);
    let res = solution::kth_smallest(data.root, data.k);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
