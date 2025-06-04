use best_time_to_buy_and_sell_stock::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("prices: {:?}", data.prices);
    let res = solution::max_profit(data.prices);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("prices: {:?}", data.prices);
    let res = solution::max_profit(data.prices);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
