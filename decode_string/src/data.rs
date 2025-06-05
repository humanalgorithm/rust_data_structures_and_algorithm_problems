#[derive(Debug)]
pub struct Data {
    pub s: String,
}

pub fn data_1() -> Data {
    Data {
        s: "3[a]2[bc]".to_string(),
    }
}

pub fn data_2() -> Data {
    Data {
        s: "3[a2[c]]".to_string(),
    }
}

pub fn data_3() -> Data {
    Data {
        s: "2[abc]3[cd]ef".to_string(),
    }
}
