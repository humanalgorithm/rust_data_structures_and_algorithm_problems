use super::data::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub fn post_order(root: Option<Rc<RefCell<TreeNode>>>, max_width: &mut Vec<i32>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let root_ref = root.as_ref().unwrap().borrow_mut();
    let left = post_order(root_ref.left.clone(), max_width);
    let right = post_order(root_ref.right.clone(), max_width);

    max_width[0] = cmp::max(max_width[0], left + right);
    return 1 + cmp::max(left, right);
}

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_width: Vec<i32> = Vec::from([0]);
    post_order(root, &mut max_width);
    return max_width[0];
}
