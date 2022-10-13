#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut result = None;
        for entry in vec.iter().rev() {
            let mut node = Self::new(*entry);
            node.next = result;
            result = Some(Box::new(node));
        }

        result
    }

    pub fn into_array(&self) -> Vec<i32> {
        let mut result = vec![self.val];

        let mut head = &self.next;
        while let Some(node) = head {
            result.push(node.val);
            head = &node.next;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_list() {
        let mut list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);

        let mut n = 1;
        while let Some(entry) = list {
            assert_eq!(entry.val, n);
            n += 1;
            list = entry.next;
        }
    }

    #[test]
    fn convert_back_to_array() {
        let list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]).unwrap();
        let ary = list.into_array();
        assert_eq!(ary, vec![1, 2, 3, 4, 5]);
    }
}
