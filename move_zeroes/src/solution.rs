pub fn move_zeroes(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut count_zeroes = 0;
    for x in 0..nums.len() {
        if nums[x] == 0 {
            count_zeroes += 1;
        }
    }
    nums.retain(|m| *m != 0);
    for _ in 0..count_zeroes {
        nums.push(0);
    }
    nums
}
