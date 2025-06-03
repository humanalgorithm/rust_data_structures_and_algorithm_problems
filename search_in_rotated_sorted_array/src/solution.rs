use std::cmp;

pub fn binary_search(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
    let mid = (right - left) / 2 + left;
    if right == left {
        if nums[left] == target {
            return left as i32;
        }
        return -1;
    }
    if nums[left] <= target && target <= nums[mid] {
        return binary_search(nums, left, mid, target);
    } else if nums[mid + 1] <= target && target <= nums[right] {
        return binary_search(nums, mid + 1, right, target);
    }
    return -1;
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 {
        return if nums[0] == target { 0 } else { -1 };
    }
    let mut pivot: usize = 0;
    let right: usize = nums.len() - 1;
    for x in 0..nums.len() - 1 {
        if nums[x + 1] < nums[x] {
            pivot = x;
        }
    }
    let res_left = binary_search(&nums, 0, pivot, target);
    let res_right = binary_search(&nums, pivot + 1, right, target);
    return cmp::max(res_left, res_right);
}
