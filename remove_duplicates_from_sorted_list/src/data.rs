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

#[derive(Debug)]
pub struct Data {
    pub head: Option<Box<ListNode>>,
}

pub fn data_1() -> Data {
    let node2 = Box::new(ListNode::new(2));
    let node1_2 = Box::new(ListNode {
        val: 1,
        next: Some(node2),
    });
    let node1_1 = Box::new(ListNode {
        val: 1,
        next: Some(node1_2),
    });

    let head = Some(node1_1);
    Data { head: head }
}

pub fn data_2() -> Data {
    let node5 = Box::new(ListNode::new(3));
    let node4 = Box::new(ListNode {
        val: 3,
        next: Some(node5),
    });
    let node3 = Box::new(ListNode {
        val: 2,
        next: Some(node4),
    });
    let node2 = Box::new(ListNode {
        val: 1,
        next: Some(node3),
    });
    let node1 = Box::new(ListNode {
        val: 1,
        next: Some(node2),
    });

    let head = Some(node1);
    Data { head: head }
}
