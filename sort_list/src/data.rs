#[derive(Clone, Debug, PartialEq, Eq)]
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
    let node4 = Box::new(ListNode::new(3));
    // Next: 1
    let node3 = Box::new(ListNode {
        val: 1,
        next: Some(node4),
    });
    // Next: 2
    let node2 = Box::new(ListNode {
        val: 2,
        next: Some(node3),
    });
    // Head: 4
    let head = Box::new(ListNode {
        val: 4,
        next: Some(node2),
    });

    Data { head: Some(head) }
}

pub fn data_2() -> Data {
    let node5 = Box::new(ListNode::new(0));
    let node4 = Box::new(ListNode {
        val: 4,
        next: Some(node5),
    });
    let node3 = Box::new(ListNode {
        val: 3,
        next: Some(node4),
    });
    let node2 = Box::new(ListNode {
        val: 5,
        next: Some(node3),
    });
    let head = Box::new(ListNode {
        val: -1,
        next: Some(node2),
    });

    Data { head: Some(head) }
}

pub fn data_3() -> Data {
    let head: Option<Box<ListNode>> = None;

    Data { head: head }
}
