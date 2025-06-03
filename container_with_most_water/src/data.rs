#[derive(Debug)]
pub struct Data {
    pub height: Vec<i32>,
}

pub fn data_1() -> Data {
    Data {
        height: vec![1, 8, 6, 2, 5, 4, 8, 3, 7],
    }
}

pub fn data_2() -> Data {
    Data { height: vec![1, 1] }
}
