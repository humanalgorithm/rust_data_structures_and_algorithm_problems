use reverse_linked_list::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("l1: {:?}", data.head);
    let res = solution::reverse_list(data.head);
    println!("\nResult: \n {:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("l1: {:?}", data.head);
    let res = solution::reverse_list(data.head);
    println!("\nResult: \n {:?}\n", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("l1: {:?}", data.head);
    let res = solution::reverse_list(data.head);
    println!("\nResult: \n {:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    println!("\nRunning Case 3...");
    case_3();
}
