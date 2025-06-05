#[derive(Debug)]
pub struct Data {
    pub coins: Vec<i32>,
    pub amount: i32,
}

pub fn data_1() -> Data {
    Data {
        coins: vec![1, 2, 5],
        amount: 11,
    }
}

pub fn data_2() -> Data {
    Data {
        coins: vec![2],
        amount: 3,
    }
}

pub fn data_3() -> Data {
    Data {
        coins: vec![1],
        amount: 0,
    }
}
