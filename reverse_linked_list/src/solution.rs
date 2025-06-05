use super::data::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head_ptr = head.clone();
    let mut prev: Option<Box<ListNode>> = None;

    while let Some(mut node) = head_ptr.take() {
        head_ptr = node.next;
        node.next = prev;
        prev = Some(node);
    }
    return prev;
}
