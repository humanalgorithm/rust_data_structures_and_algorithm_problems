#[derive(Debug)]
pub struct Data {
    pub s: Vec<char>,
}

pub fn data_1() -> Data {
    Data {
        s: vec!['h', 'e', 'l', 'l', 'o'],
    }
}

pub fn data_2() -> Data {
    Data {
        s: vec!['H', 'a', 'n', 'n', 'a', 'h'],
    }
}
