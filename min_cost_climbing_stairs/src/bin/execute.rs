use min_cost_climbing_stairs::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input: ");
    println!("costs: {:?}", data.costs);
    let res = solution::min_cost_climbing_stairs(data.costs);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input: ");
    println!("costs: {:?}", data.costs);
    let res = solution::min_cost_climbing_stairs(data.costs);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("\nRunning Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
}
