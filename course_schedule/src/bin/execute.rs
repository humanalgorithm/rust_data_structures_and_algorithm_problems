use course_schedule::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("num_courses: {:?}", data._num_courses);
    println!("prerequisites: {:?}", data.prerequisites);
    let res = solution::can_finish(data._num_courses, data.prerequisites);
    println!("\nResult:");
    println!("{:?}", res);
}

fn case_2() {
    let data = data::data_2();
    println!("Input data:");
    println!("num_courses: {:?}", data._num_courses);
    println!("prerequisites: {:?}", data.prerequisites);
    let res = solution::can_finish(data._num_courses, data.prerequisites);
    println!("\nResult:");
    println!("{:?}", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
    println!("\nRunning Case 2...");
    case_2();
}
