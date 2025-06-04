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
            *node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }
}

pub fn data_1() -> Data {
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;

    let values = [2, 1, 3];

    // Insert each value into the BST
    for &v in &values {
        insert(&mut root, v);
    }
    Data { root: root }
}

pub fn data_2() -> Data {
    // Manually construct the tree [5,1,4,null,null,3,6]
    // Structure:
    //     5
    //    / \
    //   1   4
    //      / \
    //     3   6

    let root = rc_node(5);

    // Left child of 5
    root.borrow_mut().left = Some(rc_node(1));

    // Right child of 5
    let right_node = Rc::clone(&root);
    let mut right_child_ref = right_node.borrow_mut();
    let right_child = right_child_ref.right.get_or_insert_with(|| rc_node(4));

    // Now, build subtrees
    // 4's left -> 3
    right_child.borrow_mut().left = Some(rc_node(3));
    // 4's right -> 6
    right_child.borrow_mut().right = Some(rc_node(6));

    Data {
        root: Some(Rc::clone(&root)),
    }
}
