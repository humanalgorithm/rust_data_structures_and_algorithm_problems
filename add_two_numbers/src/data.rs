#[derive(Debug)]
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

pub fn data_1_l1() -> Option<Box<ListNode>> {
    let node3 = Box::new(ListNode::new(3));
    let node2 = Box::new(ListNode {
        val: 4,
        next: Some(node3),
    });
    let head = Box::new(ListNode {
        val: 2,
        next: Some(node2),
    });
    return Some(head);
}

pub fn data_1_l2() -> Option<Box<ListNode>> {
    // Create the linked list with values [5, 6, 4]
    let node3 = Box::new(ListNode::new(4));
    let node2 = Box::new(ListNode {
        val: 6,
        next: Some(node3),
    });
    let head = Box::new(ListNode {
        val: 5,
        next: Some(node2),
    });
    Some(head)
}

pub fn data_2_empty() -> Option<Box<ListNode>> {
    let head = Box::new(ListNode::new(0));
    Some(head)
}

pub fn data_3_l1() -> Option<Box<ListNode>> {
    // Create nodes starting from the tail
    let node7 = Box::new(ListNode::new(9));
    let node6 = Box::new(ListNode {
        val: 9,
        next: Some(node7),
    });
    let node5 = Box::new(ListNode {
        val: 9,
        next: Some(node6),
    });
    let node4 = Box::new(ListNode {
        val: 9,
        next: Some(node5),
    });
    let node3 = Box::new(ListNode {
        val: 9,
        next: Some(node4),
    });
    let node2 = Box::new(ListNode {
        val: 9,
        next: Some(node3),
    });
    let head = Box::new(ListNode {
        val: 9,
        next: Some(node2),
    });
    Some(head)
}

pub fn data_3_l2() -> Option<Box<ListNode>> {
    // Create nodes starting from the tail
    let node4 = Box::new(ListNode::new(9));
    let node3 = Box::new(ListNode {
        val: 9,
        next: Some(node4),
    });
    let node2 = Box::new(ListNode {
        val: 9,
        next: Some(node3),
    });
    let head = Box::new(ListNode {
        val: 9,
        next: Some(node2),
    });
    Some(head)
}
