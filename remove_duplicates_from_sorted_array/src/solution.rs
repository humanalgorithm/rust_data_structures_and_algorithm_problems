pub fn remove_duplicates(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut indices_remove: Vec<i32> = vec![];
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[j] > nums[i] {
                break;
            }
            if nums[i] == nums[j] {
                indices_remove.push(i as i32);
                break;
            }
        }
    }
    indices_remove.reverse();
    for indices in indices_remove {
        nums.remove(indices as usize);
    }
    nums
}
