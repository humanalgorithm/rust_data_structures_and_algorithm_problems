#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
    pub target: i32,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![5, 7, 7, 8, 8, 10],
        target: 8,
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![5, 7, 7, 8, 8, 10],
        target: 6,
    }
}

pub fn data_3() -> Data {
    Data {
        nums: vec![],
        target: 0,
    }
}
