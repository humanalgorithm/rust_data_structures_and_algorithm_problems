use word_ladder::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("begin_word: {:?}", data.begin_word);
    println!("end_word: {:?}", data.end_word);
    println!("end_word: {:?}", data.word_list);
    let res = solution::ladder_length(data.begin_word, data.end_word, data.word_list);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("begin_word: {:?}", data.begin_word);
    println!("end_word: {:?}", data.end_word);
    println!("end_word: {:?}", data.word_list);
    let res = solution::ladder_length(data.begin_word, data.end_word, data.word_list);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
