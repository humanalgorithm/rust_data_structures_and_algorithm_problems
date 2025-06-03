use merge_two_sorted_lists::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("list1: {:?}", data.list1);
    println!("list2: {:?}", data.list2);
    let res = solution::merge_two_lists(data.list1, data.list2);
    println!("\nResult: \n{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("list1: {:?}", data.list1);
    println!("list2: {:?}", data.list2);
    let res = solution::merge_two_lists(data.list1, data.list2);
    println!("\nResult: \n{:?}", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("list1: {:?}", data.list1);
    println!("list2: {:?}", data.list2);
    let res = solution::merge_two_lists(data.list1, data.list2);
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
