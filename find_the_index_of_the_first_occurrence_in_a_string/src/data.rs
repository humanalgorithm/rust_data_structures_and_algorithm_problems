#[derive(Debug)]
pub struct Data {
    pub haystack: String,
    pub needle: String,
}

pub fn data_1() -> Data {
    Data {
        haystack: "sadbutsad".to_string(),
        needle: "sad".to_string(),
    }
}

pub fn data_2() -> Data {
    Data {
        haystack: "leetcode".to_string(),
        needle: "leeto".to_string(),
    }
}

// pub fn data_2() -> Data {
//     Data { n: 1 }
// }
