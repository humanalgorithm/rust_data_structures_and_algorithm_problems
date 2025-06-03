use crate::data::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;

    let mut dummy = Box::new(ListNode {
        val: -1,
        next: list1.clone(),
    });
    let mut curr = &mut dummy;

    while let (Some(node1), Some(node2)) = (&list1, &list2) {
        if node1.val <= node2.val {
            curr.next = list1.take();
            curr = curr.next.as_mut().unwrap();
            list1 = curr.next.take();
        } else {
            curr.next = list2.take();
            curr = curr.next.as_mut().unwrap();
            list2 = curr.next.take();
        }
    }
    curr.next = list1.or(list2);
    dummy.next
}
