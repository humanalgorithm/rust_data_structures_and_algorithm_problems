#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![-1, 0, 1, 2, -1, -4],
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![0, 1, 1],
    }
}

pub fn data_3() -> Data {
    Data {
        nums: vec![0, 0, 0],
    }
}
