use remove_nth_node_from_end_of_list::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("head: {:?}", data.head);
    println!("n: {:?}", data.n);
    let res = solution::remove_nth_from_end(data.head, data.n);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("head: {:?}", data.head);
    println!("n: {:?}", data.n);
    let res = solution::remove_nth_from_end(data.head, data.n);
    println!("\nResult: \n{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("head: {:?}", data.head);
    println!("n: {:?}", data.n);
    let res = solution::remove_nth_from_end(data.head, data.n);
    println!("\nResult: \n{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    println!("\nRunning Case 3...");
    case_3();
}
