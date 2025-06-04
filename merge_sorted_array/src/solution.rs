use std::collections::HashMap;

pub fn merge<'a>(
    nums1: &'a mut Vec<i32>,
    m: i32,
    nums2: &mut Vec<i32>,
    n: i32,
) -> &'a mut Vec<i32> {
    let (mut map, mut index) = (HashMap::new(), 0);
    for x in 0..m as usize {
        map.entry(nums1[x]).and_modify(|m| *m = *m + 1).or_insert(1);
    }
    for x in 0..n as usize {
        map.entry(nums2[x]).and_modify(|m| *m = *m + 1).or_insert(1);
    }
    let mut keys: Vec<&i32> = map.keys().into_iter().collect();
    keys.sort();

    for key in keys {
        for _ in 0..map[key] {
            nums1[index] = *key;
            index += 1;
        }
    }
    nums1
}
