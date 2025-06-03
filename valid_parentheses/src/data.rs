#[derive(Debug)]
pub struct Data {
    pub s: String,
}

pub fn data_1() -> Data {
    Data {
        s: "()".to_string(),
    }
}

pub fn data_2() -> Data {
    Data {
        s: "()[]{}".to_string(),
    }
}

pub fn data_3() -> Data {
    Data {
        s: "(]".to_string(),
    }
}

pub fn data_4() -> Data {
    Data {
        s: "([])".to_string(),
    }
}
