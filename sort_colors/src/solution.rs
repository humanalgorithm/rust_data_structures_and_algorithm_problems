pub fn sort_colors(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1..nums.len()).rev() {
            if nums[j] < nums[j - 1] {
                let tmp = nums[j - 1];
                nums[j - 1] = nums[j];
                nums[j] = tmp;
            }
        }
    }
    nums
}
