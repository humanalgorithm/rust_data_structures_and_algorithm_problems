use remove_duplicates_from_sorted_list::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("head: {:?}", data.head);
    let res = solution::delete_duplicates(data.head);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("head: {:?}", data.head);
    let res = solution::delete_duplicates(data.head);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
