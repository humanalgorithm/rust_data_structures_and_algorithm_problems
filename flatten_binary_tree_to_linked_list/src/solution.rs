use super::data::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<Rc<RefCell<TreeNode>>>) {
    if !node.is_some() {
        return;
    }
    stack.push(node.clone().unwrap());
    let node_ref = node.as_ref().unwrap().borrow();
    dfs(&node_ref.left, stack);
    dfs(&node_ref.right, stack);
}
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) -> &mut Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return root;
    }

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
    dfs(&*root, &mut stack);

    for x in (0..stack.len() - 1).rev() {
        let mut node = stack[x].borrow_mut();
        let prev_node = stack[x + 1].clone();
        node.left = None;
        node.right = Some(prev_node);
    }
    root
}
