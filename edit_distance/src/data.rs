pub struct Data {
    pub word1: String,
    pub word2: String,
}

pub fn data_1() -> Data {
    return Data {
        word1: "horse".to_string(),
        word2: "ros".to_string(),
    };
}

pub fn data_2() -> Data {
    return Data {
        word1: "intention".to_string(),
        word2: "execution".to_string(),
    };
}
