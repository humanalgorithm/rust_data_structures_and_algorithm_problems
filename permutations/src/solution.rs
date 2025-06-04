use std::collections::HashSet;

pub fn dfs(nums: &Vec<i32>, pre: &mut Vec<i32>, post: Vec<i32>, perms: &mut HashSet<Vec<i32>>) {
    if post.is_empty() {
        perms.insert(pre.to_vec());
        return;
    }
    for x in 0..post.len() {
        pre.push(post[x]);
        let new_post: Vec<i32> = [post[..x].to_vec(), post[x + 1..].to_vec()].concat();
        dfs(nums, pre, new_post, perms);
        pre.pop();
    }
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut perms: HashSet<Vec<i32>> = HashSet::new();
    dfs(&nums, &mut vec![], nums.clone(), &mut perms);
    let ret_value: Vec<Vec<i32>> = perms.iter().map(|m| m.to_vec()).collect();
    return ret_value;
}
