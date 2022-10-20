pub struct Solution;
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::do_level(&root, 0, &mut result);
        result
    }

    fn do_level(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<Vec<i32>>) {
        if let Some(node) = root {
            if result.len() <= level {
                result.push(Vec::new());
            }

            result[level].push(node.borrow().val);
            Self::do_level(&node.borrow().left, level + 1, result);
            Self::do_level(&node.borrow().right, level + 1, result);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&vec![3, 9, 20, i32::MIN, i32::MIN, 15, 7]);
        assert_eq!(
            Solution::level_order(tree),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&vec![1]);
        assert_eq!(Solution::level_order(tree), vec![vec![1]]);
    }

    #[test]
    fn ex3() {
        assert!(Solution::level_order(None).is_empty());
    }
}
