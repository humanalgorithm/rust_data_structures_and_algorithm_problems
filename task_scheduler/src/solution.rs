use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut map: HashMap<char, i32> = HashMap::new();

    for task in tasks {
        map.entry(task).and_modify(|m| *m = *m + 1).or_insert(1);
    }
    for value in map.values() {
        heap.push(*value);
    }
    let mut time = 0;

    while !q.is_empty() || !heap.is_empty() {
        time += 1;
        if !heap.is_empty() {
            let this_task = heap.pop().unwrap();
            if this_task - 1 != 0 {
                q.push_back((this_task - 1, time + n));
            }
        }
        if !q.is_empty() {
            match q.front() {
                Some(task) => {
                    if task.1 == time {
                        let next_task = q.pop_front().unwrap().0;
                        heap.push(next_task);
                    }
                }
                None => {}
            }
        }
    }

    return time;
}
