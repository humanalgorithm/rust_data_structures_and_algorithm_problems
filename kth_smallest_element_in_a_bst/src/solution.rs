use super::data::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut ptr = root;
    let mut n = 0;
    stack.push(ptr.clone());

    while !stack.is_empty() {
        while let Some(node) = ptr.take() {
            ptr = node.borrow().left.clone();
            stack.push(Some(node));
        }
        ptr = stack.pop().unwrap();
        n += 1;
        if n == k {
            return ptr.unwrap().borrow().val;
        }
        ptr = ptr.unwrap().borrow().right.clone();
    }
    return 0;
}
