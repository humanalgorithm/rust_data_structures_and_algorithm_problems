#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![2, 3, -2, 4],
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![-2, 0, -1],
    }
}
