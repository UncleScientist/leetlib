pub struct Solution;

// abcabcddd
//
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        for ch in s.chars() {
            *hm.entry(ch).or_insert(0) += 1;
        }

        let mut max_len = 0;
        let mut has_odd = false;
        for val in hm.values() {
            if val % 2 == 0 {
                max_len += val;
            } else {
                max_len += val - 1;
                has_odd = true;
            }
        }

        max_len + has_odd as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn triplea_b() {
        assert_eq!(Solution::longest_palindrome("aaab".to_string()), 3);
    }
}
