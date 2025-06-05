#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![10, 9, 2, 5, 3, 7, 101, 18],
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![0, 1, 0, 3, 2, 3],
    }
}

pub fn data_3() -> Data {
    Data {
        nums: vec![7, 7, 7, 7, 7, 7, 7],
    }
}
