#[derive(Debug)]
pub struct Data {
    pub strs: Vec<String>,
}

pub fn data_1() -> Data {
    Data {
        strs: vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ],
    }
}

pub fn data_2() -> Data {
    Data {
        strs: vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
    }
}
