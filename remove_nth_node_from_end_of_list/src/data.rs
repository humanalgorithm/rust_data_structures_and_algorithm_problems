#[derive(Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

#[derive(Debug)]
pub struct Data {
    pub head: Option<Box<ListNode>>,
    pub n: i32,
}

pub fn data_1() -> Data {
    // Build the linked list [1, 2, 3, 4, 5]
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
    Data {
        head: Some(head),
        n: 2,
    }
}

pub fn data_2() -> Data {
    // Create a linked list with a single node [1]
    let head = Box::new(ListNode::new(1));

    Data {
        head: Some(head),
        n: 1,
    }
}

pub fn data_3() -> Data {
    // Create nodes starting from the tail
    let node2 = Box::new(ListNode::new(2));
    let node1 = Box::new(ListNode {
        val: 1,
        next: Some(node2),
    });

    // The head of the list
    let head = Some(node1);

    // Print the list to verify
    Data { head: head, n: 1 }
}
