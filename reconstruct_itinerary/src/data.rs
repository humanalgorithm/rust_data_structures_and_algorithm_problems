#[derive(Debug)]
pub struct Data {
    pub tickets: Vec<Vec<String>>,
}

pub fn data_1() -> Data {
    Data {
        tickets: vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()],
        ],
    }
}

pub fn data_2() -> Data {
    Data {
        tickets: vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["SFO".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "JFK".to_string()],
            vec!["ATL".to_string(), "SFO".to_string()],
        ],
    }
}
