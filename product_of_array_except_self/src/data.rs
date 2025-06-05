#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![1, 2, 3, 4],
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![-1, 1, 0, -3, 3],
    }
}
