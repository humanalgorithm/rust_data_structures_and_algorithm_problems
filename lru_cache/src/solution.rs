use std::collections::HashMap;

use std::collections::VecDeque;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LRUCache {
    capacity: i32,
    map: HashMap<i32, i32>,
    q: VecDeque<i32>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        return Self {
            capacity: capacity,
            map: HashMap::with_capacity(capacity as usize),
            q: VecDeque::new(),
        };
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let index = self.q.iter().position(|x| x == &key);
            if index.is_some() {
                self.q.remove(index.unwrap());
            }
            self.q.push_front(key);
            return *self.map.get(&key).unwrap();
        }
        return -1;
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.map.entry(key).and_modify(|m| *m = value);
            let index: usize = self.q.iter().position(|x| x == &key).unwrap();
            self.q.remove(index);
            self.q.push_front(key);
        } else {
            self.map.insert(key, value);
            self.q.push_front(key);
        }
        if self.map.len() > self.capacity as usize {
            let key = self.q.pop_back().unwrap();
            self.map.remove(&key);
        }
    }
}
