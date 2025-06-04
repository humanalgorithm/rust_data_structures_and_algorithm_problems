use binary_tree_level_order_traversal::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("root: {:?}", data.root);
    println!("n: {:?}", data.root);
    let res = solution::level_order(data.root);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("root: {:?}", data.root);
    println!("n: {:?}", data.root);
    let res = solution::level_order(data.root);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("root: {:?}", data.root);
    println!("n: {:?}", data.root);
    let res = solution::level_order(data.root);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
    println!("Running Case 3...");
    case_3();
}
