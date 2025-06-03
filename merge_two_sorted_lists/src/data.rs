#[derive(PartialEq, Eq, Clone, Debug)]
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
    pub list1: Option<Box<ListNode>>,
    pub list2: Option<Box<ListNode>>,
}

pub fn data_1_list_1() -> Option<Box<ListNode>> {
    // Create nodes starting from the tail
    let node4 = Box::new(ListNode::new(4));
    let node2 = Box::new(ListNode {
        val: 2,
        next: Some(node4),
    });
    let node1 = Box::new(ListNode {
        val: 1,
        next: Some(node2),
    });

    let head = Some(node1);
    head
}

pub fn data_1_list_2() -> Option<Box<ListNode>> {
    // Create nodes starting from the tail
    let node4 = Box::new(ListNode::new(4));
    let node3 = Box::new(ListNode {
        val: 3,
        next: Some(node4),
    });
    let node1 = Box::new(ListNode {
        val: 1,
        next: Some(node3),
    });

    let head = Some(node1);
    head
}

pub fn data_1() -> Data {
    let list1 = data_1_list_1();
    let list2 = data_1_list_2();
    Data {
        list1: list1,
        list2: list2,
    }
}

pub fn data_2_list_1() -> Option<Box<ListNode>> {
    let head: Option<Box<ListNode>> = None;
    head
}

pub fn data_2_list_2() -> Option<Box<ListNode>> {
    let head: Option<Box<ListNode>> = None;
    head
}

pub fn data_2() -> Data {
    let list1 = data_2_list_1();
    let list2 = data_2_list_2();
    Data {
        list1: list1,
        list2: list2,
    }
}

pub fn data_3_list_1() -> Option<Box<ListNode>> {
    let head: Option<Box<ListNode>> = None;
    head
}

pub fn data_3_list_2() -> Option<Box<ListNode>> {
    let head = Some(Box::new(ListNode::new(0)));
    head
}

pub fn data_3() -> Data {
    let list1 = data_3_list_1();
    let list2 = data_3_list_2();
    Data {
        list1: list1,
        list2: list2,
    }
}
