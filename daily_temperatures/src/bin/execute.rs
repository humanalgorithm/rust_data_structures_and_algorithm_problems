use daily_temperatures::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input: ");
    println!("temperatures: {:?}", data.temperatures);
    let res = solution::daily_temperatures(data.temperatures);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input: ");
    println!("temperatures: {:?}", data.temperatures);
    let res = solution::daily_temperatures(data.temperatures);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input: ");
    println!("temperatures: {:?}", data.temperatures);
    let res = solution::daily_temperatures(data.temperatures);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("\nRunning Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
    println!("Running Case 3...");
    case_3();
}
