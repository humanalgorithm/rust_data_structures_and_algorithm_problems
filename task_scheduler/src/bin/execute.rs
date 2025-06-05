use task_scheduler::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("\nInput: ");
    println!("tasks: {:?}", data.tasks);
    println!("n: {:?}", data.n);
    let res = solution::least_interval(data.tasks, data.n);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("\nInput: ");
    println!("tasks: {:?}", data.tasks);
    println!("n: {:?}", data.n);
    let res = solution::least_interval(data.tasks, data.n);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_3() {
    let data = data::data_3();
    println!("\nInput: ");
    println!("tasks: {:?}", data.tasks);
    println!("n: {:?}", data.n);
    let res = solution::least_interval(data.tasks, data.n);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
    println!("Running Case 3...");
    case_3();
}
