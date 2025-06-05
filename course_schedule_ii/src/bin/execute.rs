use course_schedule_ii::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("num_courses: {:?}", data.num_courses);
    println!("prerequisites: {:?}", data.prerequisites);
    let res = solution::find_order(data.num_courses, data.prerequisites);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("num_courses: {:?}", data.num_courses);
    println!("prerequisites: {:?}", data.prerequisites);
    let res = solution::find_order(data.num_courses, data.prerequisites);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn case_3() {
    let data = data::data_3();
    println!("Input data:");
    println!("num_courses: {:?}", data.num_courses);
    println!("prerequisites: {:?}", data.prerequisites);
    let res = solution::find_order(data.num_courses, data.prerequisites);
    println!("\nResult:");
    println!("{:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("Running Case 2...");
    case_2();
    println!("Running Case 2...");
    case_3();
}
