pub struct Data {
    pub nums1: Vec<i32>,
    pub m: i32,
    pub nums2: Vec<i32>,
    pub n: i32,
}

pub fn data_1() -> Data {
    return Data {
        nums1: vec![1, 2, 3, 0, 0, 0],
        m: 3,
        nums2: vec![2, 5, 6],
        n: 3,
    };
}

pub fn data_2() -> Data {
    return Data {
        nums1: vec![1],
        m: 1,
        nums2: vec![],
        n: 0,
    };
}

pub fn data_3() -> Data {
    return Data {
        nums1: vec![0],
        m: 0,
        nums2: vec![1],
        n: 1,
    };
}
