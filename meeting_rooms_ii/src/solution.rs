// use std::collections::VecDeque;
use itertools::Itertools;
use std::cmp;

pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
    let intervals_len = intervals.len();
    let start: Vec<i32> = intervals
        .iter_mut()
        .sorted_by_key(|x| x[0])
        .map(|m| m[0])
        .collect();
    let end: Vec<i32> = intervals
        .into_iter()
        .sorted_by_key(|x| x[1])
        .map(|m| m[1])
        .collect();

    let (mut s, mut e, mut count, mut max_count) = (0, 0, 0, 0);

    while s < intervals_len {
        if start[s] < end[e] {
            count += 1;
            s += 1;
        } else {
            count -= 1;
            e += 1;
        }
        max_count = cmp::max(max_count, count);
    }
    return max_count;
}
