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
    let root = rc_node(1);
    // Left subtree
    root.borrow_mut().left = Some(rc_node(2));
    let left_child = Rc::clone(root.borrow().left.as_ref().unwrap());
    left_child.borrow_mut().left = Some(rc_node(3));
    left_child.borrow_mut().right = Some(rc_node(4));
    // Right subtree
    let root_clone = Rc::clone(&root);
    let mut right_child_ref = root_clone.borrow_mut();
    let right_child = right_child_ref.right.get_or_insert_with(|| rc_node(5));
    right_child.borrow_mut().right = Some(rc_node(6));

    Data {
        root: Some(Rc::clone(&root)),
    }
}

pub fn data_2() -> Data {
    let root: Option<Rc<RefCell<TreeNode>>> = None;

    Data { root: root }
}

pub fn data_3() -> Data {
    let root = rc_node(0);

    Data {
        root: Some(Rc::clone(&root)),
    }
}
