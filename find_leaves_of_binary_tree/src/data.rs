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
    // Create root
    let root = rc_node(1);
    // Left child of root: 2
    {
        let mut root_borrow = root.borrow_mut();
        root_borrow.left = Some(rc_node(2));
        // Children of 2
        if let Some(node2) = &root_borrow.left {
            node2.borrow_mut().left = Some(rc_node(4));
            node2.borrow_mut().right = Some(rc_node(5));
        }
        // Right child of root: 3
        root_borrow.right = Some(rc_node(3));
    }
    Data {
        root: Some(Rc::clone(&root)),
    }
}

pub fn data_2() -> Data {
    let root = rc_node(1);

    Data {
        root: Some(Rc::clone(&root)),
    }
}
