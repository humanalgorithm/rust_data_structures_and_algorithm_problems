pub struct Data {
    pub matrix: Vec<Vec<char>>,
}

pub fn data_1() -> Data {
    return Data {
        matrix: vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ],
    };
}

pub fn data_2() -> Data {
    return Data {
        matrix: vec![vec!['0', '1'], vec!['1', '0']],
    };
}

pub fn data_3() -> Data {
    return Data {
        matrix: vec![vec!['0']],
    };
}
