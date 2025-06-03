pub fn next_permutation(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut pivot = nums.len() - 1;
    let mut found = false;

    for i in (1..nums.len()).rev() {
        if nums[i] > nums[i - 1] {
            pivot = i - 1;
            found = true;
            break;
        }
    }
    if !found {
        nums.reverse();
        return nums;
    }
    let mut swap = nums.len() - 1;
    while nums[swap] <= nums[pivot] {
        swap -= 1;
    }
    let tmp = nums[swap];
    nums[swap] = nums[pivot];
    nums[pivot] = tmp;

    nums[pivot + 1..].sort();
    return nums;
}
