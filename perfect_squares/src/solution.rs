use std::cmp;
use std::collections::HashSet;
pub fn dfs(
    current: i32,
    target: i32,
    use_count: i32,
    min_use_count: &mut Vec<i32>,
    avail_squares: &Vec<i32>,
    visited: &mut HashSet<(i32, i32)>,
) {
    if min_use_count[0] < use_count {
        return;
    }
    if current < 0 {
        return;
    }
    if current == 0 {
        min_use_count[0] = cmp::min(min_use_count[0], use_count);
        return;
    }
    if visited.contains(&(current, use_count)) {
        return;
    }
    visited.insert((current, use_count));
    for num in avail_squares {
        dfs(
            current - num,
            target,
            use_count + 1,
            min_use_count,
            avail_squares,
            visited,
        );
    }
}
pub fn num_squares(n: i32) -> i32 {
    let mut squares_avail: Vec<i32> = Vec::from([1]);
    let mut min_use_count: Vec<i32> = Vec::from([i32::MAX]);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for x in 1..n / 2 + 1 {
        let this_square = x * x;
        if this_square <= n {
            squares_avail.push(this_square)
        }
    }
    squares_avail.reverse();
    dfs(n, n, 0, &mut min_use_count, &squares_avail, &mut visited);
    return min_use_count[0];
}
