pub struct Data {
    pub s: String,
    pub p: String,
}

pub fn data_1() -> Data {
    return Data {
        s: "cbaebabacd".to_string(),
        p: "abc".to_string(),
    };
}

pub fn data_2() -> Data {
    return Data {
        s: "abab".to_string(),
        p: "ab".to_string(),
    };
}
