use super::data::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if !root.is_some() {
        return vec![];
    }
    let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    let mut levels = Vec::new();
    levels.push([root.clone().unwrap().borrow().val].to_vec());
    q.push_front(root);

    while !q.is_empty() {
        let q_len = q.len();
        let mut level = Vec::new();
        for _ in 0..q_len {
            let node = q.pop_front().unwrap();
            let node_current = node.unwrap();
            let node_ref = node_current.borrow();

            if node_ref.left.is_some() {
                let val = node_ref.left.as_ref().unwrap().borrow().val;
                q.push_back(node_ref.left.clone());
                level.push(val);
            }
            if node_ref.right.is_some() {
                let val = node_ref.right.as_ref().unwrap().borrow().val;
                q.push_back(node_ref.right.clone());
                level.push(val);
            }
        }
        if !level.is_empty() {
            levels.push(level);
        }
    }
    let mut res: Vec<i32> = Vec::new();
    for level in levels {
        res.push(level[level.len() - 1]);
    }
    return res;
}
