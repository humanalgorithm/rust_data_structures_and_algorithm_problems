#[derive(Debug)]
pub struct Data {
    pub dividend: i32,
    pub divisor: i32,
}

pub fn data_1() -> Data {
    Data {
        dividend: 10,
        divisor: 3,
    }
}

pub fn data_2() -> Data {
    Data {
        dividend: 7,
        divisor: -3,
    }
}
