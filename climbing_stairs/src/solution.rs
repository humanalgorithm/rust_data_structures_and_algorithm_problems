use std::collections::HashMap;

pub fn dfs(n: i32, count: i32, solutions: &mut HashMap<i32, i32>) -> i32 {
    if count > n {
        return 0;
    }
    if count == n {
        return 1;
    }

    let step_1 = match solutions.get(&(count + 1)) {
        Some(value) => *value,
        None => dfs(n, count + 1, solutions),
    };
    let step_2 = match solutions.get(&(count + 2)) {
        Some(value) => *value,
        None => dfs(n, count + 2, solutions),
    };

    solutions.insert(count, step_1 + step_2);
    return step_1 + step_2;
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut solutions: HashMap<i32, i32> = HashMap::new();
    return dfs(n, 0, &mut solutions);
}
