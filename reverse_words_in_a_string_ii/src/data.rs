#[derive(Debug)]
pub struct Data {
    pub s: Vec<char>,
}

pub fn data_1() -> Data {
    Data {
        s: vec![
            't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
        ],
    }
}

pub fn data_2() -> Data {
    Data { s: vec!['a'] }
}
