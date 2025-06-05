use std::cmp;

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut costs = cost;
    costs.push(0);

    for i in (0..costs.len() - 3).rev() {
        costs[i] += cmp::min(costs[i + 1], costs[i + 2]);
    }
    return cmp::min(costs[0], costs[1]);
}
