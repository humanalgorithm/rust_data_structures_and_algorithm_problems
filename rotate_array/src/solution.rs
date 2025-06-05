pub fn shift_by_one(nums: &mut Vec<i32>) {
    let mut prev: i32 = nums[nums.len() - 1];
    for x in 0..nums.len() {
        let tmp = nums[x];
        nums[x] = prev;
        prev = tmp;
    }
}
pub fn rotate(nums: &mut Vec<i32>, k: i32) -> &mut Vec<i32> {
    let mut k = k;
    k = k % nums.len() as i32;
    for _ in 0..k {
        shift_by_one(nums);
    }
    nums
}
