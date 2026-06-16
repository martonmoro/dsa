// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.

// According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined 
// between two nodes p and q as the lowest node in T that has both p and q as descendants 
// (where we allow a node to be a descendant of itself).”

use std::rc::Rc;
use std::cell::RefCell;

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

fn lca(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let pv = p.as_ref().unwrap().borrow().val;
    let qv = q.as_ref().unwrap().borrow().val;

    fn helper(node: Option<Rc<RefCell<TreeNode>>>, pv: i32, qv: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => None, // empty subtree found nothing
            Some(n) => {
                // Base case: this node is one of the targets.
                {
                    let val = n.borrow().val;
                    if val == pv || val == qv {
                        return Some(n.clone());
                    }
                }

                // Recurse into both children
                let left = helper(n.borrow().left.clone(), pv, qv);
                let right = helper(n.borrow().right.clone(), pv, qv);

                // Combine left and right
                // - both Some -> this node is the LCA
                // - only one Some -> return that one
                // - both None -> return None
                match (left, right) {
                    (Some(_), Some(_)) => Some(n.clone()), // split point → LCA
                    (Some(l), None)    => Some(l),
                    (None, Some(r))    => Some(r),
                    (None, None)       => None,
                }
            }
        }
    }

    helper(root, pv, qv)
}

///////////////////////////////////////////

fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;

    // Build nodes
    let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let n6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let n0 = Rc::new(RefCell::new(TreeNode::new(0)));
    let n8 = Rc::new(RefCell::new(TreeNode::new(8)));
    let n7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(4)));

    // Connect tree
    n3.borrow_mut().left = Some(n5.clone());
    n3.borrow_mut().right = Some(n1.clone());

    n5.borrow_mut().left = Some(n6);
    n5.borrow_mut().right = Some(n2.clone());

    n1.borrow_mut().left = Some(n0);
    n1.borrow_mut().right = Some(n8);

    n2.borrow_mut().left = Some(n7);
    n2.borrow_mut().right = Some(n4.clone());

    let root = Some(n3.clone());

    // -------------------------
    // Test 1: LCA(5, 1) => 3
    // -------------------------
    let result = lca(root.clone(), Some(n5.clone()), Some(n1.clone()));
    println!("LCA(5, 1) = {}", result.unwrap().borrow().val);

    // -------------------------
    // Test 2: LCA(5, 4) => 5
    // -------------------------
    let result = lca(root, Some(n5.clone()), Some(n4.clone()));
    println!("LCA(5, 4) = {}", result.unwrap().borrow().val);
}