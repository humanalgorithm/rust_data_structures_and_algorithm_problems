use std::cmp;

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut lis: Vec<i32> = vec![1; nums.len()];
    let mut max_val = 1;

    for i in (0..nums.len()).rev() {
        for j in i + 1..nums.len() {
            if nums[j] > nums[i] {
                lis[i] = cmp::max(lis[i], 1 + lis[j]);
                max_val = cmp::max(max_val, lis[i]);
            }
        }
    }
    return max_val;
}
