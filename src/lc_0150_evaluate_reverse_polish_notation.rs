pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for t in tokens {
            if let Ok(val) = t.parse::<i32>() {
                stack.push(val);
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match t.as_str() {
                    "+" => stack.push(a + b),
                    "-" => stack.push(a - b),
                    "/" => stack.push(a / b),
                    "*" => stack.push(a * b),
                    _ => panic!("oops"),
                }
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        )
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
