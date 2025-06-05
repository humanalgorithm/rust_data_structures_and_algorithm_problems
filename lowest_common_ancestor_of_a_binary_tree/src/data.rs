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
    pub p: Option<Rc<RefCell<TreeNode>>>,
    pub q: Option<Rc<RefCell<TreeNode>>>,
}
// Helper to create Rc<RefCell<TreeNode>>
fn rc_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

pub fn data_1() -> Data {
    // Root node 3
    let root = rc_node(3);
    // Level 1 children: 5 (left), 1 (right)
    let p = rc_node(5);
    root.borrow_mut().left = Some(p.clone());
    let q = rc_node(1);
    root.borrow_mut().right = Some(q.clone());

    // Children of 5: 6 (left), 2 (right)
    if let Some(node5) = &root.borrow().left {
        node5.borrow_mut().left = Some(rc_node(6));
        node5.borrow_mut().right = Some(rc_node(2));
        // Children of 2: 7 (left), 4 (right)
        if let Some(node2) = &node5.borrow().right {
            node2.borrow_mut().left = Some(rc_node(7));
            node2.borrow_mut().right = Some(rc_node(4));
        }
    }
    // Children of 1: 0 (left), 8 (right)
    if let Some(node1) = &root.borrow().right {
        node1.borrow_mut().left = Some(rc_node(0));
        node1.borrow_mut().right = Some(rc_node(8));
    }

    Data {
        root: Some(root),
        p: Some(p),
        q: Some(q),
    }
}

pub fn data_2() -> Data {
    // Root node 3
    let root = rc_node(3);
    // Level 1 children: 5 (left), 1 (right)
    let p = rc_node(5);
    root.borrow_mut().left = Some(p.clone());
    root.borrow_mut().right = Some(rc_node(1));

    // Children of 5: 6 (left), 2 (right)
    let q = rc_node(4);
    if let Some(node5) = &root.borrow().left {
        node5.borrow_mut().left = Some(rc_node(6));
        node5.borrow_mut().right = Some(rc_node(2));
        // Children of 2: 7 (left), 4 (right)
        if let Some(node2) = &node5.borrow().right {
            node2.borrow_mut().left = Some(rc_node(7));
            node2.borrow_mut().right = Some(q.clone());
        }
    }
    // Children of 1: 0 (left), 8 (right)
    if let Some(node1) = &root.borrow().right {
        node1.borrow_mut().left = Some(rc_node(0));
        node1.borrow_mut().right = Some(rc_node(8));
    }

    Data {
        root: Some(root),
        p: Some(p),
        q: Some(q),
    }
}

pub fn data_3() -> Data {
    // Create root node 1
    let p = rc_node(1);
    let root = p.clone();

    // 2 as right child
    let q = rc_node(2);
    root.borrow_mut().right = Some(q.clone());

    Data {
        root: Some(root),
        p: Some(p),
        q: Some(q),
    }
}
