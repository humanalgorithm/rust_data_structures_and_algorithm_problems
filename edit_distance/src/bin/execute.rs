use edit_distance::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("word1: {:?}", data.word1);
    println!("word2: {:?}", data.word2);
    let res = solution::min_distance(data.word1, data.word2);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("word1: {:?}", data.word1);
    println!("word2: {:?}", data.word2);
    let res = solution::min_distance(data.word1, data.word2);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
