use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, 1);
    let (mut curr_sum, mut total) = (0, 0);

    for num in nums {
        curr_sum += num;
        let diff = curr_sum - k;
        if map.contains_key(&diff) {
            total += map.get(&diff).unwrap();
        }
        map.entry(curr_sum).and_modify(|m| *m = *m + 1).or_insert(1);
    }
    return total;
}
