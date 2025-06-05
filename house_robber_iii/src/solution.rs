use super::data::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }
    let root_ref = root.as_ref().unwrap().borrow_mut();
    let left_pair = dfs(root_ref.left.clone());
    let right_pair = dfs(root_ref.right.clone());

    let with_root = root_ref.val + left_pair.1 + right_pair.1;
    let without_root = cmp::max(left_pair.0, left_pair.1) + cmp::max(right_pair.0, right_pair.1);

    return (with_root, without_root);
}
pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let pair = dfs(root);
    return cmp::max(pair.0, pair.1);
}
