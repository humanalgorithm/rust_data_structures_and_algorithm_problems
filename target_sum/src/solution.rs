use std::collections::HashMap;
pub fn backtrack(
    nums: &Vec<i32>,
    mut dp: &mut HashMap<(usize, i32), i32>,
    index: usize,
    current: i32,
    target: i32,
) -> i32 {
    if index == nums.len() {
        return if current == target { 1 } else { 0 };
    }
    if dp.get(&(index, current)).is_some() {
        return dp[&(index, current)];
    }
    let res = backtrack(nums, &mut dp, index + 1, current + nums[index], target)
        + backtrack(nums, &mut dp, index + 1, current - nums[index], target);
    dp.insert((index, current), res);
    return dp[&(index, current)];
}
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp: HashMap<(usize, i32), i32> = HashMap::new();
    backtrack(&nums, &mut dp, 0, 0, target);
    return dp[&(0, 0)];
}
