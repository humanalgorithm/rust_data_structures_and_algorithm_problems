use meeting_rooms_ii::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("intervals: {:?}", data.intervals);
    let res = solution::min_meeting_rooms(data.intervals);
    println!("Result is {:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("intervals: {:?}", data.intervals);
    let res = solution::min_meeting_rooms(data.intervals);
    println!("Result is {:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
    // println!("\nRunning Case 3...");
    // case_3();
}
