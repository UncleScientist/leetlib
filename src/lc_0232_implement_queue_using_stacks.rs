#[derive(Default)]
pub struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    pub fn new() -> Self {
        Self {
            stack1: vec![],
            stack2: vec![],
        }
    }

    pub fn push(&mut self, x: i32) {
        while let Some(val) = self.stack1.pop() {
            self.stack2.push(val);
        }
        self.stack1.push(x);
        while let Some(val) = self.stack2.pop() {
            self.stack1.push(val);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.stack1.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        *self.stack1.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
        assert_eq!(q.pop(), 2);
        assert!(q.empty());
    }
}
