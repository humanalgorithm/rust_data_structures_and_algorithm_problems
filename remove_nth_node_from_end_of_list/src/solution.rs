use crate::data::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });

    let mut fast = dummy.clone();

    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    let mut slow = dummy.as_mut();
    while fast.next.is_some() {
        fast = fast.next.unwrap();
        slow = slow.next.as_mut().unwrap();
    }

    let next = slow.next.as_mut().unwrap();
    slow.next = next.next.clone();
    dummy.next
}
