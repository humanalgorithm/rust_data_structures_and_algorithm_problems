use std::cmp;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut curr_sum = i32::MIN;

    for num in nums {
        if curr_sum < 0 {
            curr_sum = 0;
        }
        curr_sum += num;
        max_sum = cmp::max(curr_sum, max_sum);
    }
    return max_sum;
}
