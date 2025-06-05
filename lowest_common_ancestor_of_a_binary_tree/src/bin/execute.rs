use lowest_common_ancestor_of_a_binary_tree::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("root: {:?}\n", data.root);
    println!("p: {:?}\n", data.p);
    println!("q: {:?}\n", data.q);
    let res = solution::lowest_common_ancestor(data.root, data.p, data.q);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("root: {:?}\n", data.root);
    println!("p: {:?}\n", data.p);
    println!("q: {:?}\n", data.q);
    let res = solution::lowest_common_ancestor(data.root, data.p, data.q);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_3() {
    let data = data::data_3();
    println!("root: {:?}\n", data.root);
    println!("p: {:?}\n", data.p);
    println!("q: {:?}\n", data.q);
    let res = solution::lowest_common_ancestor(data.root, data.p, data.q);
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
