use super::data::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

pub fn dfs(
    mut root: Option<Rc<RefCell<TreeNode>>>,
    layer: i32,
    map: &mut HashMap<i32, Vec<i32>>,
) -> i32 {
    if root.is_none() {
        return layer;
    }
    let root_ref = root.as_mut().unwrap().borrow_mut();
    let left = dfs(root_ref.left.clone(), layer, map);
    let right = dfs(root_ref.right.clone(), layer, map);

    let max_layer = cmp::max(left, right);
    map.entry(max_layer)
        .and_modify(|m| m.push(root_ref.val))
        .or_insert(vec![root_ref.val]);

    return max_layer + 1;
}

pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    dfs(root, 0, &mut map);

    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut keys: Vec<i32> = map.keys().map(|m| *m as i32).collect();
    keys.sort();
    for key in keys {
        res.push(map[&key].to_vec());
    }
    return res;
}
