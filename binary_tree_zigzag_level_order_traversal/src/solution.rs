use super::data::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    queue.push_front(root);
    let mut forward: bool = true;

    while !queue.is_empty() {
        let level_len = queue.len();
        let mut level: Vec<i32> = Vec::new();
        for _ in 0..level_len {
            let node = queue.pop_front();
            if let Some(current) = node.unwrap() {
                let ref_node = current.borrow();
                level.push(ref_node.val);
                queue.push_back(ref_node.left.clone());
                queue.push_back(ref_node.right.clone());
            }
        }

        if !level.is_empty() {
            if forward == false {
                level.reverse();
            }
            forward = !forward;
            result.push(level);
        }
    }
    return result;
}
