use reverse_words_in_a_string_ii::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("s is {:?}", data.s);
    let res = solution::reverse_words(&mut data.s);
    println!("\nResult: \n{:?}\n", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("s is {:?}", data.s);
    let res = solution::reverse_words(&mut data.s);
    println!("\nResult: \n{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 1...");
    case_2();
}
