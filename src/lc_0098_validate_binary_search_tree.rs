pub struct Solution;
use crate::TreeNode;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate_tree(&root)
    }

    fn validate_tree(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            Self::is_less(&node.borrow().left, node.borrow().val)
                && Self::is_greater(&node.borrow().right, node.borrow().val)
                && Self::validate_tree(&node.borrow().left)
                && Self::validate_tree(&node.borrow().right)
        } else {
            true
        }
    }

    fn is_less(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        if let Some(node) = node {
            val > node.borrow().val
                && Self::is_less(&node.borrow().left, val)
                && Self::is_less(&node.borrow().right, val)
        } else {
            true
        }
    }

    fn is_greater(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        if let Some(node) = node {
            val < node.borrow().val
                && Self::is_greater(&node.borrow().left, val)
                && Self::is_greater(&node.borrow().right, val)
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(Solution::is_valid_bst(TreeNode::from_vec(&[2, 1, 3])));
    }

    #[test]
    fn ex2() {
        assert!(!Solution::is_valid_bst(TreeNode::from_vec(&[
            5,
            1,
            4,
            i32::MAX,
            i32::MAX,
            3,
            6
        ])));
    }
}
