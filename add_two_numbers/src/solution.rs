use super::data::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut current = &mut head;
    let (mut l1, mut l2) = (l1, l2);
    let mut carry = 0;

    loop {
        if !(l1.is_some() || l2.is_some() || carry == 1) {
            break;
        }
        let (l1_val, l1_next) = match l1 {
            Some(l1_node) => (l1_node.val, l1_node.next),
            None => (0, None),
        };
        let (l2_val, l2_next) = match l2 {
            Some(l2_node) => (l2_node.val, l2_node.next),
            None => (0, None),
        };
        (l1, l2) = (l1_next, l2_next);

        let value = l1_val + l2_val + carry;
        let digit = value % 10;
        carry = if value >= 10 { 1 } else { 0 };
        current.next = Some(Box::new(ListNode::new(digit)));
        current = current.next.as_mut().unwrap();
    }
    return head.next;
}
