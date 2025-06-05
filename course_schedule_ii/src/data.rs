pub struct Data {
    pub num_courses: i32,
    pub prerequisites: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        num_courses: 2,
        prerequisites: vec![vec![1, 0]],
    };
}

pub fn data_2() -> Data {
    return Data {
        num_courses: 4,
        prerequisites: vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]],
    };
}

pub fn data_3() -> Data {
    return Data {
        num_courses: 1,
        prerequisites: vec![],
    };
}
