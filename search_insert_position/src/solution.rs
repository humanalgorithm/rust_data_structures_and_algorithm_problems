pub fn binary_search(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
    let mid = (right - left) / 2 + left;

    if target <= nums[left] {
        return left as i32;
    } else if right == nums.len() - 1 {
        if nums[right] == target {
            return right as i32;
        }
        if target > nums[right] {
            return right as i32 + 1;
        }
    }

    if right - left == 1 {
        if nums[left] <= target && target <= nums[right] {
            return left as i32 + 1;
        }
        return -1;
    }
    //go left
    else if nums[left] <= target && target <= nums[mid] {
        let result = binary_search(&nums, left, mid, target);
        if result > -1 {
            return result;
        }
    }
    //go right
    else if nums[mid + 1] <= target && target <= nums[right] {
        let result = binary_search(&nums, mid + 1, right, target);
        if result > -1 {
            return result;
        }
    }
    // mid
    else if nums[mid] <= target && target < nums[mid + 1] {
        return mid as i32 + 1;
    }
    return -1;
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let result = binary_search(&nums, 0, nums.len() - 1, target);
    return result;
}
