
pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    let mut top_line: i32 = 1;
    let mut num_zeroes = 0;
    let all_zeroes = vec![0; nums.len()];

    for x in 0..nums.len() {
        if nums[x] == 0 {
            num_zeroes += 1;
            if num_zeroes > 1 {
                return all_zeroes;
            }
            continue;
        }
        top_line *= nums[x];
    }
    for x in 0..nums.len() {
        if num_zeroes == 1 && nums[x] != 0 {
            nums[x] = 0;
            continue;
        } else if nums[x] == 0 {
            nums[x] = top_line;
        } else {
            nums[x] = top_line / nums[x];
        }
    }
    return nums;
}
