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
// Helper to create Rc<RefCell<TreeNode>>
fn rc_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

pub fn data_1() -> Data {
    let root = rc_node(3);
    // Left child 9
    root.borrow_mut().left = Some(rc_node(9));

    // Right child 20
    let right_node_clone = Rc::clone(&root);
    let mut right_node_ref = right_node_clone.borrow_mut();
    let right_node = right_node_ref.right.get_or_insert_with(|| rc_node(20));

    // 20's left child 15
    right_node.borrow_mut().left = Some(rc_node(15));
    // 20's right child 7
    right_node.borrow_mut().right = Some(rc_node(7));

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

pub fn data_3() -> Data {
    let root: Option<Rc<RefCell<TreeNode>>> = None;

    Data { root: root }
}
