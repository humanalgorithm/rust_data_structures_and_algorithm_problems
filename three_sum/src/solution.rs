use std::collections::HashSet;
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut l = 0;
    let mut nums = nums;
    nums.sort();
    let mut output_set: HashSet<Vec<i32>> = HashSet::new();
    let nums_len = nums.len();

    while l < nums_len - 2 {
        let mut m = l + 1;
        let mut r = nums_len - 1;
        while m < r {
            let total = nums[l] + nums[m] + nums[r];
            if total == 0 {
                output_set.insert(vec![nums[l], nums[m], nums[r]]);
                while m < r && nums[m] == nums[m + 1] {
                    m += 1;
                }
                while r > m && nums[r] == nums[r - 1] {
                    r -= 1;
                }
                m += 1;
            } else if total > 0 {
                r -= 1;
            } else {
                m += 1;
            }
        }
        l += 1;
    }
    return output_set.into_iter().collect();
}
