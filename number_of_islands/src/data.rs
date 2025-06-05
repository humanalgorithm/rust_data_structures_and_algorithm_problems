pub struct Data {
    pub grid: Vec<Vec<char>>,
}

pub fn data_1() -> Data {
    return Data {
        grid: vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ],
    };
}

pub fn data_2() -> Data {
    return Data {
        grid: vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ],
    };
}
