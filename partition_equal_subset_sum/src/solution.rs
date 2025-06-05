use std::collections::HashSet;
pub fn can_partition(nums: Vec<i32>) -> bool {
    let total_sum: i32 = nums.iter().map(|&i| i).sum();
    let target = total_sum / 2;
    if total_sum % 2 != 0 {
        return false;
    }
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(0);

    for x in (0..nums.len()).rev() {
        for item in set.clone().iter() {
            if nums[x] + item == target {
                return true;
            }
            set.insert(nums[x] + item);
        }
    }
    if set.contains(&target) {
        return true;
    }
    return false;
}
