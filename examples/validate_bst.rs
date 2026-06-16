// Given the root of a binary tree, determine if it is a valid binary search tree (BST).

// A valid BST is defined as follows:

// The left subtree of a node contains only nodes with keys strictly less than the node's key.
// The right subtree of a node contains only nodes with keys strictly greater than the node's key.
// Both the left and right subtrees must also be binary search trees.

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

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // bounds are exclusive; None means "no bound on that side"
    fn check(node: Option<Rc<RefCell<TreeNode>>>, min: Option<i64>, max: Option<i64>) -> bool {
        match node {
            None => true, // empty subtree is always valid
            Some(n) => {
                let n = n.borrow();
                let v = n.val as i64;

                if let Some(lo) = min {
                    if v <= lo { return false; }
                }
                if let Some(hi) = max {
                    if v >= hi { return false; }
                }

                // left subtree: max tightens to v; right subtree: min tightens to v
                check(n.left.clone(),  min, Some(v))
                    && check(n.right.clone(), Some(v), max)
            }
        }
    }

    check(root, None, None)
}

//////////////////////////////////////////

fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;

    // -------------------------
    // VALID BST
    //       2
    //      / \
    //     1   3
    // -------------------------
    let root_valid = Rc::new(RefCell::new(TreeNode::new(2)));
    let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(3)));

    root_valid.borrow_mut().left = Some(n1);
    root_valid.borrow_mut().right = Some(n3);

    println!("Valid BST? {}", is_valid_bst(Some(root_valid))); // true

    // -------------------------
    // INVALID BST
    //        5
    //       / \
    //      3   8
    //         / \
    //        4   10   ❌ 4 breaks BST rule
    // -------------------------
    let root_invalid = Rc::new(RefCell::new(TreeNode::new(5)));

    let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n8 = Rc::new(RefCell::new(TreeNode::new(8)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let n10 = Rc::new(RefCell::new(TreeNode::new(10)));

    root_invalid.borrow_mut().left = Some(n3);
    root_invalid.borrow_mut().right = Some(n8.clone());
    n8.borrow_mut().left = Some(n4);
    n8.borrow_mut().right = Some(n10);

    println!(
        "Invalid BST? {}",
        is_valid_bst(Some(root_invalid))
    ); // false
}