pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let array = s.chars().collect::<Vec<char>>();

        let mut hs = HashSet::new();

        let mut start = 0;
        let mut end = 0;
        let mut maxlen = 0;
        while end < array.len() {
            if !hs.insert(array[end]) {
                while array[start] != array[end] {
                    hs.remove(&array[start]);
                    start += 1;
                }
                start += 1;
            }

            end += 1;

            maxlen = maxlen.max(hs.len());
        }

        maxlen as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn wrong_answer_1() {
        assert_eq!(
            Solution::length_of_longest_substring("aabaab!bb".to_string()),
            3
        );
    }
}
