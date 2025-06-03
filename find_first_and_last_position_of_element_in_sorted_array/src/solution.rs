pub fn binary_search_right(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
    let mid = (right - left) / 2 + left;
    if right - left == 0 {
        if nums[left] == target {
            return left as i32;
        }
        return -1;
    }
    if nums[mid + 1] <= target && target <= nums[right] {
        let right_res = binary_search_right(&nums, mid + 1, right, target);
        if right_res > -1 {
            return right_res;
        }
    } else if nums[left] <= target && target <= nums[mid] {
        let left_res = binary_search_right(&nums, left, mid, target);
        if left_res > -1 {
            return left_res;
        }
    }
    return -1;
}

pub fn binary_search_left(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
    let mid = (right - left) / 2 + left;
    if right - left == 0 {
        if nums[left] == target {
            return left as i32;
        }
        return -1;
    }
    if nums[left] <= target && target <= nums[mid] {
        let left_left = binary_search_left(&nums, left, mid, target);
        if left_left > -1 {
            return left_left;
        }
    } else if nums[mid + 1] <= target && target <= nums[right] {
        let left_right = binary_search_left(&nums, mid + 1, right, target);
        if left_right > -1 {
            return left_right;
        }
    }
    return -1;
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_len = nums.len();
    if nums_len == 0 {
        return vec![-1, -1];
    }
    let left = binary_search_left(&nums, 0, nums_len - 1, target);
    let right = binary_search_right(&nums, 0, nums_len - 1, target);

    return [left, right].to_vec();
}
