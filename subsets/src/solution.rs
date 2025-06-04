use std::collections::HashSet;

pub fn dfs(
    index: usize,
    curr_path: &mut Vec<i32>,
    avail_path: &Vec<i32>,
    paths: &mut HashSet<Vec<i32>>,
) {
    for x in index..avail_path.len() {
        curr_path.push(avail_path[x]);
        paths.insert(curr_path.clone());
        let sub_avail = [avail_path[0..x].to_vec(), avail_path[x + 1..].to_vec()].concat();
        dfs(x, curr_path, &sub_avail, paths);
        curr_path.pop();
    }
}
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut paths: HashSet<Vec<i32>> = HashSet::from([(vec![])]);
    dfs(0, &mut vec![], &nums, &mut paths);
    return paths.iter().map(|m| m.to_vec()).collect();
}
