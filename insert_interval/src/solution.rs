use std::cmp;

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.push(new_interval);
    intervals.sort_by(|v1, v2| v1[0].cmp(&v2[0]));
    let mut stack: Vec<Vec<i32>> = vec![];

    for interval in intervals {
        if stack.is_empty() {
            stack.push(interval);
            continue;
        }
        let prev = stack.pop().unwrap();

        if interval[0] <= prev[1] && interval[1] >= prev[0] {
            let lesser = cmp::min(prev[0], interval[0]);
            let greater = cmp::max(prev[1], interval[1]);
            stack.push(vec![lesser, greater]);
            continue;
        }
        stack.push(prev);
        stack.push(interval);
    }
    return stack;
}
