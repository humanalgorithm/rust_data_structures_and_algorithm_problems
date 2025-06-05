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
    let root = rc_node(1);

    // Left subtree of 1
    let left_child = Rc::clone(&root);
    left_child.borrow_mut().left = None; // null, so skip
    left_child.borrow_mut().right = Some(rc_node(2));

    // 2's right child 5
    if let Some(node2) = &left_child.borrow().right {
        node2.borrow_mut().left = None; // null
        node2.borrow_mut().right = Some(rc_node(5));
    }

    // Right subtree of 1
    let root_clone = Rc::clone(&root);
    let mut root_ref = root_clone.borrow_mut();
    let right_child = root_ref.right.get_or_insert_with(|| rc_node(3));
    right_child.borrow_mut().left = None; // null
    right_child.borrow_mut().right = Some(rc_node(4));

    Data {
        root: Some(Rc::clone(&root)),
    }
}

pub fn data_2() -> Data {
    let root = rc_node(1);

    // Left subtree
    let left = Rc::clone(&root);
    left.borrow_mut().left = Some(rc_node(2));
    // 2's left child: 4
    if let Some(node2) = &left.borrow().left {
        node2.borrow_mut().left = Some(rc_node(4));
        // 4's left child: 5
        if let Some(node4) = &node2.borrow().left {
            node4.borrow_mut().left = Some(rc_node(5));
        }
    }

    // Right subtree: 3 (leaf)
    left.borrow_mut().right = Some(rc_node(3));

    Data {
        root: Some(Rc::clone(&root)),
    }
}

pub fn data_3() -> Data {
    let root = rc_node(1);
    // No left child
    // Right child 3
    root.borrow_mut().right = Some(rc_node(3));

    Data {
        root: Some(Rc::clone(&root)),
    }
}

pub fn data_4() -> Data {
    let root: Option<Rc<RefCell<TreeNode>>> = None;
    Data { root: root }
}
