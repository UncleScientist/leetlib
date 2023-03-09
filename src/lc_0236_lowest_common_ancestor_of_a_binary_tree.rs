use crate::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_node(&root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }

    fn find_node(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if node.borrow().val == p || node.borrow().val == q {
                return root.clone();
            }
            let left = Self::find_node(&node.borrow().left, p, q);
            let right = Self::find_node(&node.borrow().right, p, q);
            match (&left, &right) {
                (None, None) => None,
                (None, _) => right,
                (_, None) => left,
                _ => root.clone(),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&vec![3, 5, 1, 6, 2, 0, 8, i32::MIN, i32::MIN, 7, 4]);
        let p = TreeNode::from_vec(&vec![5]);
        let q = TreeNode::from_vec(&vec![1]);
        let result = Solution::lowest_common_ancestor(tree, p, q);
        assert_eq!(3, result.unwrap().borrow().val);
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&vec![3, 5, 1, 6, 2, 0, 8, i32::MIN, i32::MIN, 7, 4]);
        let p = TreeNode::from_vec(&vec![5]);
        let q = TreeNode::from_vec(&vec![4]);
        let result = Solution::lowest_common_ancestor(tree, p, q);
        assert_eq!(5, result.unwrap().borrow().val);
    }

    #[test]
    fn ex3() {
        let tree = TreeNode::from_vec(&vec![1, 2]);
        let p = TreeNode::from_vec(&vec![1]);
        let q = TreeNode::from_vec(&vec![2]);
        let result = Solution::lowest_common_ancestor(tree, p, q);
        assert_eq!(1, result.unwrap().borrow().val);
    }
}
