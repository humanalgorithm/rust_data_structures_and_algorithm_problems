#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
    pub val: i32,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![3, 2, 2, 3],
        val: 3,
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![0, 1, 2, 2, 3, 0, 4, 2],
        val: 2,
    }
}
