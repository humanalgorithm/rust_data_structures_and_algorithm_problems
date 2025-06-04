use std::collections::HashSet;

pub fn dfs(
    nums_len: usize,
    pre: &mut Vec<i32>,
    post: &mut Vec<i32>,
    perms: &mut HashSet<Vec<i32>>,
) {
    if pre.len() == nums_len {
        perms.insert(pre.to_vec());
        return;
    }

    for x in 0..post.len() {
        pre.push(post[x]);
        let mut new_post = [post[..x].to_vec(), post[x + 1..].to_vec()].concat();
        dfs(nums_len, pre, &mut new_post, perms);
        pre.pop();
    }
}
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut perms: HashSet<Vec<i32>> = HashSet::new();
    dfs(nums.len(), &mut vec![], &mut nums, &mut perms);
    let res: Vec<Vec<i32>> = perms.iter().map(|m| m.to_vec()).collect();
    return res;
}
