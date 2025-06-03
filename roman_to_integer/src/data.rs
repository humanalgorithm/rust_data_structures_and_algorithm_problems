#[derive(Debug)]
pub struct Data {
    pub s: String,
}

pub fn data_1() -> Data {
    Data {
        s: "III".to_string(),
    }
}

pub fn data_2() -> Data {
    Data {
        s: "LVIII".to_string(),
    }
}

pub fn data_3() -> Data {
    Data {
        s: "MCMXCIV".to_string(),
    }
}
