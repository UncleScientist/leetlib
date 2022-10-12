use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    invert(&root)
}

fn invert(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = node {
        let right = invert(&node.borrow().left);
        let left = invert(&node.borrow().right);

        Some(Rc::new(RefCell::new(TreeNode {
            val: node.borrow().val,
            left,
            right,
        })))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&[4, 2, 7, 1, 3, 6, 9]);
        let inversion = invert_tree(tree);
        assert_eq!(
            inversion.unwrap().borrow().into_array(),
            vec![9, 7, 6, 4, 3, 2, 1]
        );
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&[2, 1, 3]);
        let inversion = invert_tree(tree);
        assert_eq!(inversion.unwrap().borrow().into_array(), vec![3, 2, 1]);
    }

    #[test]
    fn ex3() {
        assert!(invert_tree(None).is_none());
    }
}
