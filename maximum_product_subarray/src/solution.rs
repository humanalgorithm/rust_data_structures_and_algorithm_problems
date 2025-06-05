use std::cmp;

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let (mut curr_min, mut curr_max) = (1, 1);

    for x in 0..nums.len() {
        let this_num = nums[x];
        if this_num == 0 {
            (curr_min, curr_max) = (1, 1);
        }
        let tmp = curr_max * this_num;
        curr_max = cmp::max(this_num * curr_max, this_num);
        curr_max = cmp::max(this_num * curr_min, curr_max);
        curr_min = cmp::min(this_num * curr_min, this_num);
        curr_min = cmp::min(curr_min, tmp);
        res = cmp::max(res, curr_max);
    }
    return res;
}
