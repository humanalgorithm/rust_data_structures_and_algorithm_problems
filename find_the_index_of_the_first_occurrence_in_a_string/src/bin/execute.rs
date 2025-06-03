use find_the_index_of_the_first_occurrence_in_a_string::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("haystack: {:?}", data.haystack);
    println!("needle: {:?}", data.needle);
    let res = solution::str_str(data.haystack, data.needle);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("haystack: {:?}", data.haystack);
    println!("needle: {:?}", data.needle);
    let res = solution::str_str(data.haystack, data.needle);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
