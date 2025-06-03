use std::collections::HashSet;
pub fn dfs(
    candidates: &Vec<i32>,
    backtrack: &mut Vec<i32>,
    combinations: &mut HashSet<Vec<i32>>,
    target: i32,
    curr_sum: i32,
) {
    if curr_sum == target {
        combinations.insert(backtrack.to_vec());
        return;
    }
    if curr_sum > target {
        return;
    }

    for x in 0..candidates.len() {
        backtrack.push(candidates[x]);
        let new_sum = curr_sum + candidates[x];
        dfs(
            &candidates[x..].to_vec(),
            backtrack,
            combinations,
            target,
            new_sum,
        );
        backtrack.pop();
    }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut combinations: HashSet<Vec<i32>> = HashSet::new();
    let mut backtrack: Vec<i32> = vec![];
    dfs(&candidates, &mut backtrack, &mut combinations, target, 0);

    return combinations.iter().map(|v| v.to_vec()).collect();
}
