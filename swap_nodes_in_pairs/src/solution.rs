use super::data::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head.clone(),
    });
    let mut curr = &mut dummy;

    while let Some(mut p) = curr.next.take() {
        if let Some(mut q) = p.next.take() {
            p.next = q.next;
            q.next = Some(p);
            curr.next = Some(q);
            curr = curr.next.as_mut().unwrap().next.as_mut().unwrap();
        } else {
            curr.next = Some(p);
            curr = curr.next.as_mut().unwrap();
        }
    }

    return dummy.next;
}
