// Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

    // Seed the queue with the root
    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_vals: Vec<i32> = Vec::new();

        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            let node = node.borrow(); // read access to this node

            level_vals.push(node.val);

            if let Some(child) = node.left.clone() {
                queue.push_back(child);
            }

            if let Some(child) = node.right.clone() {
                queue.push_back(child);
            }
        }

        result.push(level_vals);
    }

    result
}

fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;

    // Build the tree:
    //       1
    //      / \
    //     2   3
    //    / \   \
    //   4   5   6

    let root = Rc::new(RefCell::new(TreeNode::new(1)));

    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));

    // connect children
    root.borrow_mut().left = Some(node2.clone());
    root.borrow_mut().right = Some(node3.clone());

    node2.borrow_mut().left = Some(node4.clone());
    node2.borrow_mut().right = Some(node5.clone());

    node3.borrow_mut().right = Some(node6.clone());

    // run level order traversal
    let result = level_order(Some(root));

    println!("{:?}", result);
}