// You are given the head of a singly linked-list. The list can be represented as:

// L0 → L1 → … → Ln - 1 → Ln
// Reorder the list to be on the following form:

// L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
// You may not modify the values in the list's nodes. Only nodes themselves may be changed.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None}
    }
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    // 1. Measure the list
    let mut len = 0;
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        len += 1;
        cur = node.next.as_ref();
    }

    if len <= 2 {
        return;
    }

    // 2. Split into two halves
    // first half keeps ceil(len/2)  nodes
    let first_len = (len+1) / 2;
    let mut tail_of_first = head.as_mut();
    for _ in 1..first_len {
        tail_of_first = tail_of_first.unwrap().next.as_mut();
    }

    // .take() detaches the second half AND sets first half's last `next` to None
    let mut second = tail_of_first.unwrap().next.take();

    // 3. Reverse the second half
    let mut reversed: Option<Box<ListNode>> = None;
    while let Some(mut node) = second {
        second = node.next.take();
        node.next = reversed;
        reversed = Some(node)
    }

    // 4. Merge halves
    let mut first = head.take();
    let mut second = reversed;
    let mut tail = head;
    let mut from_first = true;
    loop {
        let node = if from_first { first.take() } else { second.take() };
        match node {
            Some(mut node) => {
                // detach the rest of whichever list we pulled from
                if from_first {
                    first = node.next.take();
                } else {
                    second = node.next.take();
                }
                *tail = Some(node); // attach the node
                tail = &mut tail.as_mut().unwrap().next; // advance tail pointer
                from_first = !from_first;
            }
            None => break,
        }
    }
}

////////////////////////////////////

// helper: build list from vector
fn build_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for &v in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }

    head
}

// helper: print list
fn print_list(head: &Option<Box<ListNode>>) {
    let mut cur = head.as_ref();

    while let Some(node) = cur {
        print!("{} -> ", node.val);
        cur = node.next.as_ref();
    }

    println!("None");
}

fn main() {
    let mut head = build_list(vec![1, 2, 3, 4, 5]);

    println!("Before:");
    print_list(&head);

    reorder_list(&mut head);

    println!("After:");
    print_list(&head);
}