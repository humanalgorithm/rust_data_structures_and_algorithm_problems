use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::with_capacity(nums.len());
    for (x, num) in nums.iter().enumerate() {
        let lookup = target - num;

        if hm.contains_key(&lookup) {
            return vec![*hm.get(&lookup).unwrap(), x as i32];
        }
        hm.insert(num, x as i32);
    }
    return nums;
}
