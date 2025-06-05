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
    // Left child 2
    {
        let mut r_borrow = root.borrow_mut();
        r_borrow.left = Some(rc_node(2));
        // 2's children: 4 and 5
        if let Some(left) = &r_borrow.left {
            left.borrow_mut().left = Some(rc_node(4));
            left.borrow_mut().right = Some(rc_node(5));
        }
        // Right child of root: 3
        r_borrow.right = Some(rc_node(3));
    }

    Data {
        root: Some(Rc::clone(&root)),
    }
}

pub fn data_2() -> Data {
    // Create root node with value 1
    let root = rc_node(1);
    // Set right child to 2
    root.borrow_mut().right = Some(rc_node(2));

    Data {
        root: Some(Rc::clone(&root)),
    }
}
