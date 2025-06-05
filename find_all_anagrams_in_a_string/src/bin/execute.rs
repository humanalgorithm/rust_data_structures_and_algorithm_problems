use find_all_anagrams_in_a_string::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("s: {:?}", data.s);
    println!("p: {:?}\n", data.p);
    let res = solution::find_anagrams(data.s, data.p);
    println!("Result: \n{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("s: {:?}", data.s);
    println!("p: {:?}\n", data.p);
    let res = solution::find_anagrams(data.s, data.p);
    println!("Result: \n{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
