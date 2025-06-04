use std::cmp;
use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let map: HashSet<_> = nums.into_iter().collect();
    let mut max_length = 0;

    for &num in &map {
        if !map.contains(&(num - 1)) {
            let mut length = 0;
            while map.contains(&(num + length)) {
                length += 1;
                max_length = cmp::max(max_length, length as i32);
            }
        }
    }
    return max_length;
}
