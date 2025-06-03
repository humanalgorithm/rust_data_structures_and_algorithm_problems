#[derive(Debug)]
pub struct Data {
    pub s: String,
}

pub fn data_1() -> Data {
    Data {
        s: "42".to_string(),
    }
}

pub fn data_2() -> Data {
    Data {
        s: "   -042".to_string(),
    }
}

pub fn data_3() -> Data {
    Data {
        s: "1337c0d3".to_string(),
    }
}

pub fn data_4() -> Data {
    Data {
        s: "0-1".to_string(),
    }
}

pub fn data_5() -> Data {
    Data {
        s: "words and 987".to_string(),
    }
}
