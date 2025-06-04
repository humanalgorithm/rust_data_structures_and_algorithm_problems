use flatten_binary_tree_to_linked_list::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("root: {:?}", data.root);
    let res = solution::flatten(&mut data.root);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("root: {:?}", data.root);
    let res = solution::flatten(&mut data.root);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_3() {
    let mut data = data::data_3();
    println!("Input data:");
    println!("root: {:?}", data.root);
    let res = solution::flatten(&mut data.root);
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
