pub struct Data {
    pub tasks: Vec<char>,
    pub n: i32,
}

pub fn data_1() -> Data {
    return Data {
        tasks: vec!['A', 'A', 'A', 'B', 'B', 'B'],
        n: 2,
    };
}

pub fn data_2() -> Data {
    return Data {
        tasks: vec!['A', 'C', 'A', 'B', 'D', 'B'],
        n: 1,
    };
}

pub fn data_3() -> Data {
    return Data {
        tasks: vec!['A', 'A', 'A', 'B', 'B', 'B'],
        n: 3,
    };
}
