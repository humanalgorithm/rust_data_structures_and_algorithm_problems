pub struct Data {
    pub paths: Vec<String>,
}

pub fn data_1() -> Data {
    return Data {
        paths: vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
            "root/c 3.txt(abcd)".to_string(),
            "root/c/d 4.txt(efgh)".to_string(),
            "root 4.txt(efgh)".to_string(),
        ],
    };
}

pub fn data_2() -> Data {
    return Data {
        paths: vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
            "root/c 3.txt(abcd)".to_string(),
            "root/c/d 4.txt(efgh)".to_string(),
        ],
    };
}
