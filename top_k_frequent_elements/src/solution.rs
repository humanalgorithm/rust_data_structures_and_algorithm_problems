use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut res = vec![];

    for num in nums {
        map.entry(num).and_modify(|m| *m += 1).or_insert(1);
    }

    for (key, value) in map.into_iter() {
        heap.push((value, key));
    }

    for _ in 0..k {
        res.push(heap.pop().unwrap().1);
    }
    return res;
}
