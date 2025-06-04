use std::cmp;

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut res: i32 = 0;

    while r < nums.len() - 1 {
        let mut farthest = 0;
        for i in l..(r + 1) {
            farthest = cmp::max(farthest, i + nums[i] as usize);
        }
        l = r + 1;
        r = farthest;
        res = res + 1;
    }
    return res;
}
