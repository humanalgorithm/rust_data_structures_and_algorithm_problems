use super::data::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }
    let mut root = TreeNode::new(preorder[0]);
    let mid = inorder.iter().position(|&r| r == preorder[0]).unwrap();
    root.left = build_tree(preorder[1..mid + 1].to_vec(), inorder[..mid].to_vec());
    root.right = build_tree(preorder[mid + 1..].to_vec(), inorder[mid + 1..].to_vec());
    return Some(Rc::new(RefCell::new(root)));
}
