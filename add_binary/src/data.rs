pub struct Data {
    pub a: String,
    pub b: String,
}

pub fn data_1() -> Data {
    return Data {
        a: "11".to_string(),
        b: "1".to_string(),
    };
}

pub fn data_2() -> Data {
    return Data {
        a: "1010".to_string(),
        b: "1011".to_string(),
    };
}
