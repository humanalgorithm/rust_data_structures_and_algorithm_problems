#[derive(Debug)]
pub struct Data {
    pub x: i32,
}

pub fn data_1() -> Data {
    Data { x: 123 }
}

pub fn data_2() -> Data {
    Data { x: -123 }
}

pub fn data_3() -> Data {
    Data { x: 120 }
}
