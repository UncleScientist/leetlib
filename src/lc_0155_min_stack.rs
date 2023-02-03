#[derive(Default)]
pub struct MinStack {
    stack: Vec<i32>,
    minstack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.minstack.is_empty() || *self.minstack.last().unwrap() >= val {
            self.minstack.push(val);
        }
    }

    pub fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        if !self.minstack.is_empty() && *self.minstack.last().unwrap() == val {
            self.minstack.pop();
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.minstack.last().unwrap()
    }
}

/*
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(-3, min_stack.get_min()); // return -3
        min_stack.pop();
        assert_eq!(0, min_stack.top()); // return 0
        assert_eq!(-2, min_stack.get_min()); // return -2
    }
}
