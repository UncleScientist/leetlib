pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words: HashSet<&str> = HashSet::from_iter(word_dict.iter().map(|s| s.as_str()));
        let mut possible = vec![false; s.len() + 1];
        possible[0] = true;

        for end in 1..=s.len() {
            for start in 0..end {
                if possible[start] && words.contains(&s[start..end]) {
                    possible[end] = true;
                }
            }
        }

        possible[s.len()]

        // Self::can_break(s.as_str(), &word_dict)
    }

    fn _can_break(s: &str, word_dict: &[String]) -> bool {
        if s.is_empty() {
            return true;
        }

        for w in word_dict {
            if s.starts_with(w) && Self::_can_break(&s[w.len()..], word_dict) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
    }

    #[test]
    fn ex2() {
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()],
        ));
    }

    #[test]
    fn ex3() {
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dogs".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ],
        ));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::word_break(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"
                .to_string(),
            vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string(),
                "aaaaa".to_string(),
                "aaaaaa".to_string(),
                "aaaaaaa".to_string(),
                "aaaaaaaa".to_string(),
                "aaaaaaaaa".to_string(),
                "aaaaaaaaaa".to_string()
            ]
        ));
    }
}
