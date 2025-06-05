use super::data::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn dfs_count(
    mut root: Option<Rc<RefCell<TreeNode>>>,
    current: i32,
    target_sum: i32,
    total_count: &mut Vec<i32>,
) {
    if root.is_none() {
        return;
    }

    let root_ref = root.as_mut().unwrap().borrow();
    match current.checked_add(root_ref.val) {
        None => return,
        _ => {}
    }
    dfs_count(
        root_ref.left.clone(),
        current + root_ref.val,
        target_sum,
        total_count,
    );
    dfs_count(
        root_ref.right.clone(),
        current + root_ref.val,
        target_sum,
        total_count,
    );

    if root_ref.val + current == target_sum {
        total_count[0] += 1;
    }
}

pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, total_count: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    let root_ref = root.as_ref().unwrap().borrow();
    dfs_count(root.clone(), 0, target_sum, total_count);
    dfs(root_ref.left.clone(), target_sum, total_count);
    dfs(root_ref.right.clone(), target_sum, total_count);
}
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    let mut total_count: Vec<i32> = Vec::from([0]);
    dfs(root, target_sum, &mut total_count);

    return total_count[0];
}
