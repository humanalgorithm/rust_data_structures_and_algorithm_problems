#[derive(Debug)]
pub struct Data {
    pub s: String,
    pub t: String,
}

pub fn data_1() -> Data {
    Data {
        s: "anagram".to_string(),
        t: "nagaram".to_string(),
    }
}

pub fn data_2() -> Data {
    Data {
        s: "rat".to_string(),
        t: "car".to_string(),
    }
}
