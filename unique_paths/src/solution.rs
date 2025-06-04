use std::cmp;
use std::collections::HashMap;

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut map: HashMap<(usize, usize), i32> = HashMap::new();

    let m = m as usize;
    let n = n as usize;

    for i in 0..m {
        for j in 0..n {
            let left = map.get(&(i, (j as i32 - 1) as usize));
            let above = map.get(&((i as i32 - 1) as usize, j));
            let mut path_count = 0;
            if left.is_some() {
                path_count += left.unwrap();
            }
            if above.is_some() {
                path_count += above.unwrap();
            }
            map.insert((i, j), cmp::max(path_count, 1));
        }
    }
    return *map.get(&(m - 1, n - 1)).unwrap();
}
