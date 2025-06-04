use super::data::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub fn in_order(node: &Option<Rc<RefCell<TreeNode>>>, visited: &mut Vec<i32>) {
    let Some(current) = node.as_ref().map(|m| m.borrow()) else {
        return;
    };
    in_order(&current.left, visited);
    visited.push(current.val);
    in_order(&current.right, visited);
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut visited: Vec<i32> = vec![];
    in_order(&root, &mut visited);
    let result: Vec<bool> = visited
        .windows(2)
        .map(|m| m[0] < m[1])
        .filter(|n| *n == false)
        .collect();
    return result.is_empty();
}
