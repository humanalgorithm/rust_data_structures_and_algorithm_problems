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
}

fn rc_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

pub fn data_1() -> Data {
    let root = rc_node(3);

    // Left subtree: node 2 with right child 3
    {
        let mut root_borrow = root.borrow_mut();
        root_borrow.left = Some(rc_node(2));
        if let Some(left_node) = &root_borrow.left {
            left_node.borrow_mut().right = Some(rc_node(3));
        }
    }

    // Right subtree: node 3 with right child 1
    {
        let mut root_borrow = root.borrow_mut();
        root_borrow.right = Some(rc_node(3));
        if let Some(right_node) = &root_borrow.right {
            right_node.borrow_mut().right = Some(rc_node(1));
        }
    }
    Data {
        root: Some(Rc::clone(&root)),
    }
}
pub fn data_2() -> Data {
    // Root node 3
    let root = rc_node(3);
    {
        let mut root_borrow = root.borrow_mut();
        // Left subtree
        root_borrow.left = Some(rc_node(4));
        if let Some(left_node) = &root_borrow.left {
            left_node.borrow_mut().left = Some(rc_node(1));
            left_node.borrow_mut().right = Some(rc_node(3));
        }
        // Right subtree
        root_borrow.right = Some(rc_node(5));
        if let Some(right_node) = &root_borrow.right {
            right_node.borrow_mut().right = Some(rc_node(1));
        }
    }
    Data {
        root: Some(Rc::clone(&root)),
    }
}
