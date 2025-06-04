
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut last_reachable = nums.len() - 1;

    for x in (0..nums.len()).rev() {
        if (nums[x] as usize + x) >= last_reachable {
            last_reachable = x;
        }
    }

    return last_reachable == 0;
}
