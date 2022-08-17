//1290. Convert Binary Number in a Linked List to Integer

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut next = head;
    let mut binary_vec = Vec::new();
    while let Some(n) = next {
        binary_vec.push(n.val.to_string());
        next = n.next;
    }
    i32::from_str_radix(&binary_vec.join(""), 2).expect("Error")
}

fn main() {
    let input1 = Some(Box::new(ListNode { val: 1, next: None }));
    let input2 = Some(Box::new(ListNode {
        val: 0,
        next: input1,
    }));
    let input3 = Some(Box::new(ListNode {
        val: 1,
        next: input2,
    }));
    println!("{}", get_decimal_value(input3));
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
