use std::cmp;

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.insert(0, 0);
    nums.insert(0, 0);
    for x in 2..nums.len() {
        nums[x] = cmp::max(nums[x] + nums[x - 2], nums[x - 1]);
    }
    return nums[nums.len() - 1];
}
