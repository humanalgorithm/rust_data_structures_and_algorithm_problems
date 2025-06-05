use super::data::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }

    let root_val = root.as_ref().unwrap().borrow().val;
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;

    if root_val == p_val || root_val == q_val {
        return root;
    }

    let l = lowest_common_ancestor(
        root.as_ref().unwrap().borrow().left.clone(),
        p.clone(),
        q.clone(),
    );
    let r = lowest_common_ancestor(root.as_ref().unwrap().borrow().right.clone(), p, q);

    if l.is_some() && r.is_some() {
        return root;
    }
    if l.is_some() { return l } else { return r }
}
