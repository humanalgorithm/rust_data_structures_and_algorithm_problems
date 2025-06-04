use std::cmp;

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut stack: Vec<Vec<i32>> = vec![];
    let mut intervals = intervals;
    intervals.sort_by(|v1, v2| v1[0].cmp(&v2[0]));

    for interval in intervals {
        if stack.is_empty() {
            stack.push(interval);
            continue;
        }
        let prev = stack.pop().unwrap();

        if prev[0] <= interval[1] && interval[0] <= prev[1] {
            let lesser = cmp::min(interval[0], prev[0]);
            let greater = cmp::max(interval[1], prev[1]);
            stack.push(vec![lesser, greater]);
            continue;
        }
        stack.push(prev);
        stack.push(interval);
    }
    return stack;
}
