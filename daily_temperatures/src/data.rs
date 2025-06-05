pub struct Data {
    pub temperatures: Vec<i32>,
}

pub fn data_1() -> Data {
    return Data {
        temperatures: vec![73, 74, 75, 71, 69, 72, 76, 73],
    };
}

pub fn data_2() -> Data {
    return Data {
        temperatures: vec![30, 40, 50, 60],
    };
}

pub fn data_3() -> Data {
    return Data {
        temperatures: vec![30, 60, 90],
    };
}
