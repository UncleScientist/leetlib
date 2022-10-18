use crate::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::diameter(&root)
    }

    fn diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_height = Self::height(&node.borrow().left);
            let right_height = Self::height(&node.borrow().right);

            let left_diam = Self::diameter(&node.borrow().left);
            let right_diam = Self::diameter(&node.borrow().right);

            (left_height + right_height).max(left_diam.max(right_diam))
        } else {
            0
        }
    }

    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            Self::height(left).max(Self::height(right)) + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 3);
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&vec![1, 2]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 1);
    }

    /*
     *                           4
     *              7                            3
     *      -              -              -9              -3
     *                                9      -7       -4      -
     *                              6   -   -6 -6    -  -
     *                             0 6     5 - 9 -

    #[test]
    fn wrong_answer_1() {
        let tree = TreeNode::from_vec(&vec![
            4,
            -7,
            -3,
            i32::MIN,
            i32::MIN,
            -9,
            -3,
            9,
            -7,
            -4,
            i32::MIN,
            6,
            i32::MIN,
            -6,
            -6,
            i32::MIN,
            i32::MIN,
            0,
            6,
            5,
            i32::MIN,
            9,
            i32::MIN,
            i32::MIN,
            -1,
            -4,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            -2,
        ]);
        println!("{}", tree.as_ref().unwrap().borrow());
        assert_eq!(Solution::diameter_of_binary_tree(tree), 8);
    }
    */
}
