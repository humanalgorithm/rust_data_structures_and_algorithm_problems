use construct_binary_tree_from_preorder_and_inorder_traversal::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("preorder: {:?}", data.preorder);
    println!("inorder: {:?}", data.inorder);
    let res = solution::build_tree(data.preorder, data.inorder);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("preorder: {:?}", data.preorder);
    println!("inorder: {:?}", data.inorder);
    let res = solution::build_tree(data.preorder, data.inorder);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
