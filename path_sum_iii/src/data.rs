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
    pub target_sum: i32,
}

fn rc_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

pub fn data_1() -> Data {
    let root = rc_node(10);
    // Level 1
    {
        let mut root_borrow = root.borrow_mut();
        root_borrow.left = Some(rc_node(5));
        root_borrow.right = Some(rc_node(-3));
    }
    // Left subtree of root: 5
    if let Some(left) = &root.borrow().left {
        let mut left_borrow = left.borrow_mut();
        left_borrow.left = Some(rc_node(3));
        left_borrow.right = Some(rc_node(2));
        // 3's children: 3, -2
        if let Some(node3a) = &left_borrow.left {
            node3a.borrow_mut().left = Some(rc_node(3));
            node3a.borrow_mut().right = Some(rc_node(-2));
        }
        // 2's child: 1
        if let Some(node2) = &left_borrow.right {
            node2.borrow_mut().right = Some(rc_node(1));
        }
    }
    // Right subtree of root: -3
    if let Some(right) = &root.borrow().right {
        right.borrow_mut().right = Some(rc_node(11));
    }

    Data {
        root: Some(Rc::clone(&root)),
        target_sum: 8,
    }
}

pub fn data_2() -> Data {
    // Root
    let root = rc_node(5);

    // Level 1
    {
        let mut root_borrow = root.borrow_mut();
        root_borrow.left = Some(rc_node(4));
        root_borrow.right = Some(rc_node(8));
    }

    // Left subtree of root (node 4)
    if let Some(left) = &root.borrow().left {
        let mut left_borrow = left.borrow_mut();
        left_borrow.left = Some(rc_node(11));
        // 11's children: 7, 2
        if let Some(node11) = &left_borrow.left {
            node11.borrow_mut().left = Some(rc_node(7));
            node11.borrow_mut().right = Some(rc_node(2));
        }
    }

    // Right subtree of root (node 8)
    if let Some(right) = &root.borrow().right {
        let mut right_borrow = right.borrow_mut();
        right_borrow.left = Some(rc_node(13));
        right_borrow.right = Some(rc_node(4));
        // Children of 13: 5, 1
        if let Some(node13) = &right_borrow.left {
            node13.borrow_mut().left = Some(rc_node(5));
            node13.borrow_mut().right = Some(rc_node(1));
        }
        // Children of 4 (right): can be added here if needed
    }
    Data {
        root: Some(Rc::clone(&root)),
        target_sum: 22,
    }
}
