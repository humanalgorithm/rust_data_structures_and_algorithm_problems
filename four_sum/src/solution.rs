use std::collections::HashSet;

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let nums_len = nums.len();
    if nums_len < 4 {
        return vec![];
    }
    nums.sort();
    let mut l1 = 0;
    let mut output: HashSet<Vec<i32>> = HashSet::new();
    while l1 < nums_len - 3 {
        if l1 > 0 && nums[l1] == nums[l1 - 1] {
            l1 += 1;
            continue;
        }
        let mut l2 = l1 + 1;
        while l2 < nums_len - 2 {
            let mut r1 = l2 + 1;
            let mut r2 = nums_len - 1;
            while r1 < r2 {
                let total = match nums[l1].checked_add(nums[l2]) {
                    Some(res1) => match res1.checked_add(nums[r1]) {
                        Some(res2) => match res2.checked_add(nums[r2]) {
                            Some(res3) => res3,
                            None => {
                                r2 -= 1;
                                continue;
                            }
                        },
                        None => {
                            r2 -= 1;
                            continue;
                        }
                    },
                    None => {
                        r2 -= 1;
                        continue;
                    }
                };
                if total == target {
                    let new_entry = vec![nums[l1], nums[l2], nums[r1], nums[r2]];
                    output.insert(new_entry);
                    r1 += 1;
                    r2 -= 1;
                    while r1 < r2 && nums[r1] == nums[r1 - 1] {
                        r1 += 1;
                    }
                    while r1 < r2 && nums[r2] == nums[r2 + 1] {
                        r2 -= 1;
                    }
                } else if total < target {
                    r1 += 1;
                } else if total > target {
                    r2 -= 1;
                }
            }
            l2 += 1;
        }
        l1 += 1;
    }
    return output.iter().map(|m| m.to_vec()).collect();
}
