use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RandomizedSet {
    // set: HashSet<i32>,
    map: HashMap<i32, usize>,
    vec: Vec<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
            vec: Vec::new(),
        };
    }

    pub fn insert(&mut self, val: i32) -> bool {
        let exists = self.map.contains_key(&val);
        if exists {
            return false;
        }

        self.map.insert(val, self.vec.len());
        self.vec.push(val);

        return true;
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }

        let last_index = self.vec.len() - 1;
        let last_val = self.vec[last_index];

        let swap_index = *self.map.get(&val).unwrap();
        self.vec[swap_index] = last_val;

        self.vec.pop();
        self.map.insert(last_val, swap_index);
        self.map.remove(&val);
        return true;
    }

    pub fn get_random(&mut self) -> i32 {
        let rand_index = rand::rng().random_range(0..self.vec.len());
        return self.vec[rand_index];
    }
}
