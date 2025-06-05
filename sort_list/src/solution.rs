use super::data::ListNode;

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head_ptr = head.clone();
    let mut node_stack: Vec<(i32, Box<ListNode>)> = Vec::new();

    while let Some(mut node) = head_ptr.take() {
        head_ptr = node.next.take();
        node_stack.push((node.val, node.clone()));
    }
    node_stack.sort_by_key(|m| m.0);

    while !node_stack.is_empty() {
        let mut this_node = node_stack.pop().unwrap();
        this_node.1.next = head_ptr.take();
        head_ptr = Some(this_node.1);
    }
    return head_ptr;
}
