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
fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

// Insert value into BST
fn insert(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
    match node {
        Some(n) => {
            if val < n.borrow().val {
                insert(&mut n.borrow_mut().left, val);
            } else {
                insert(&mut n.borrow_mut().right, val);
            }
        }
        None => {
            *node = Some(new_node(val));
        }
    }
}

pub fn data_1() -> Data {
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;
    let data = [4, 2, 7, 1, 3, 6, 9];

    // Build BST
    for &v in &data {
        insert(&mut root, v);
    }

    Data { root: root }
}

pub fn data_2() -> Data {
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;
    let data = [2, 1, 3];

    // Build the BST
    for &v in &data {
        insert(&mut root, v);
    }

    Data { root: root }
}

pub fn data_3() -> Data {
    let root: Option<Rc<RefCell<TreeNode>>> = None;
    Data { root: root }
}
