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
    pub preorder: Vec<i32>,
    pub inorder: Vec<i32>,
}

pub fn data_1() -> Data {
    Data {
        preorder: vec![3, 9, 20, 15, 7],
        inorder: vec![9, 3, 15, 20, 7],
    }
}

pub fn data_2() -> Data {
    Data {
        preorder: vec![-1],
        inorder: vec![-1],
    }
}
