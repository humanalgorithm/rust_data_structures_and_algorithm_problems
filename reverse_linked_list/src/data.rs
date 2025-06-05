#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

#[derive(Debug)]
pub struct Data {
    pub head: Option<Box<ListNode>>,
}

pub fn data_1() -> Data {
    // Build the list from tail to head
    let node5 = Box::new(ListNode::new(5));
    let node4 = Box::new(ListNode {
        val: 4,
        next: Some(node5),
    });
    let node3 = Box::new(ListNode {
        val: 3,
        next: Some(node4),
    });
    let node2 = Box::new(ListNode {
        val: 2,
        next: Some(node3),
    });
    let head = Box::new(ListNode {
        val: 1,
        next: Some(node2),
    });

    Data { head: Some(head) }
}

pub fn data_2() -> Data {
    let node2 = Box::new(ListNode::new(2));
    // Then the head: 1
    let head = Box::new(ListNode {
        val: 1,
        next: Some(node2),
    });
    Data { head: Some(head) }
}

pub fn data_3() -> Data {
    let head: Option<Box<ListNode>> = None;
    Data { head: head }
}
