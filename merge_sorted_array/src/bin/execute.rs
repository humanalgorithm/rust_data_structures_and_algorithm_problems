use merge_sorted_array::{data, solution};

fn case_1() {
    let mut data = data::data_1();
    println!("Input data:");
    println!("nums1: {:?}", data.nums1);
    println!("m: {:?}", data.m);
    println!("nums2: {:?}", data.nums2);
    println!("n: {:?}\n", data.n);
    let res = solution::merge(&mut data.nums1, data.m, &mut data.nums2, data.n);
    println!("Result:");
    println!("{:?}\n", res);
}

fn case_2() {
    let mut data = data::data_2();
    println!("Input data:");
    println!("nums1: {:?}", data.nums1);
    println!("m: {:?}", data.m);
    println!("nums2: {:?}", data.nums2);
    println!("n: {:?}\n", data.n);
    let res = solution::merge(&mut data.nums1, data.m, &mut data.nums2, data.n);
    println!("Result:");
    println!("{:?}\n", res);
}

fn case_3() {
    let mut data = data::data_3();
    println!("Input data:");
    println!("nums1: {:?}", data.nums1);
    println!("m: {:?}", data.m);
    println!("nums2: {:?}", data.nums2);
    println!("n: {:?}\n", data.n);
    let res = solution::merge(&mut data.nums1, data.m, &mut data.nums2, data.n);
    println!("Result:");
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
