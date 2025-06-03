use std::collections::HashSet;

pub fn dfs(
    candidates: &mut Vec<i32>,
    backtrack: &mut Vec<i32>,
    target: i32,
    combos: &mut HashSet<Vec<i32>>,
    sum: i32,
) {
    if sum == target {
        combos.insert(backtrack.to_vec());
        return;
    }
    for x in 0..candidates.len() {
        if x != 0 && candidates[x] == candidates[x - 1] {
            continue;
        }
        backtrack.push(candidates[x]);
        let new_sum = sum + candidates[x];
        if new_sum > target {
            backtrack.pop();
            break;
        }
        dfs(
            &mut candidates[x + 1..].to_vec(),
            backtrack,
            target,
            combos,
            new_sum,
        );
        backtrack.pop();
    }
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();

    let mut combos: HashSet<Vec<i32>> = HashSet::new();
    let mut backtrack: Vec<i32> = vec![];
    dfs(&mut candidates, &mut backtrack, target, &mut combos, 0);

    let res: Vec<Vec<i32>> = combos.iter().map(|l| l.to_vec()).collect();
    return res;
}
