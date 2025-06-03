pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> &mut Vec<i32> {
    let (mut array_len, mut index) = (nums.len(), 0);

    while index < array_len {
        if nums[index] == val {
            nums.remove(index);
            array_len -= 1;
        } else {
            index += 1;
        }
    }
    nums
}
