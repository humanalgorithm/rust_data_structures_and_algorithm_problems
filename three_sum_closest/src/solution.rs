pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut nums = nums;
    nums.sort();
    let nums_len = nums.len();
    let mut closest_target = i32::MAX;

    while l < nums_len - 2 {
        let mut m = l + 1;
        let mut r = nums_len - 1;
        while m < r {
            let total = nums[l] + nums[m] + nums[r];
            let difference = target - total;
            let difference_abs = difference.abs();

            if difference_abs == 0 {
                return total;
            }
            if difference_abs < (target - closest_target).abs() {
                closest_target = total;
            }
            if total < target {
                while m < r && nums[m] == nums[m + 1] {
                    m += 1;
                }
                m += 1;
            } else if total > target {
                while r > m && nums[r] == nums[r - 1] {
                    r -= 1;
                }
                r -= 1;
            }
        }
        l += 1;
    }
    return closest_target;
}
