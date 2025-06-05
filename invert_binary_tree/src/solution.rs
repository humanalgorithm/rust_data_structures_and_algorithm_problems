use super::data::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if !root.is_some() {
        return root;
    }
    let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

    q.push_front(root.clone());

    while !q.is_empty() {
        for _ in 0..q.len() {
            let this_node = q.pop_front().unwrap().unwrap();
            let mut node_ref = this_node.borrow_mut();

            let tmp = node_ref.left.clone();
            node_ref.left = node_ref.right.clone();
            node_ref.right = tmp;
            if node_ref.left.is_some() {
                q.push_back(node_ref.left.clone());
            }
            if node_ref.right.is_some() {
                q.push_back(node_ref.right.clone());
            }
        }
    }
    return root;
}
