use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub root: Option<Rc<RefCell<TreeNode>>>,
    pub k: i32,
}
// Helper to create Rc<RefCell<TreeNode>>
fn rc_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

pub fn data_1() -> Data {
    // Build the root node with value 3
    let root = rc_node(3);
    // Left child 1
    root.borrow_mut().left = Some(rc_node(1));
    // Right child 4
    root.borrow_mut().right = Some(rc_node(4));

    // 1's right child 2
    if let Some(left) = &root.borrow().left {
        left.borrow_mut().right = Some(rc_node(2));
    }

    Data {
        root: Some(root),
        k: 1,
    }
}

pub fn data_2() -> Data {
    let root = rc_node(5);
    // Left child 3
    root.borrow_mut().left = Some(rc_node(3));
    // Right child 6
    root.borrow_mut().right = Some(rc_node(6));

    // 3's children: 2 and 4
    if let Some(node3) = &root.borrow().left {
        node3.borrow_mut().left = Some(rc_node(2));
        node3.borrow_mut().right = Some(rc_node(4));
        // 2's left child 1
        if let Some(node2) = &node3.borrow().left {
            node2.borrow_mut().left = Some(rc_node(1));
        }
    }
    Data {
        root: Some(root),
        k: 3,
    }
}
