pub struct Data {
    pub _num_courses: i32,
    pub prerequisites: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        _num_courses: 2,
        prerequisites: vec![vec![1, 0]],
    };
}

pub fn data_2() -> Data {
    return Data {
        _num_courses: 2,
        prerequisites: vec![vec![1, 0], vec![0, 1]],
    };
}
