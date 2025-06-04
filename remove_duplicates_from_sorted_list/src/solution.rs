use super::data::ListNode;

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head_ptr = head.as_mut();

    while let Some(node) = head_ptr {
        let mut next_node = node.next.take();

        while let Some(next_iter) = next_node.as_mut() {
            if node.val == next_iter.val {
                next_node = next_iter.next.take();
            } else {
                node.next = next_node.take();
                break;
            }
        }
        head_ptr = node.next.as_mut();
    }
    return head;
}
