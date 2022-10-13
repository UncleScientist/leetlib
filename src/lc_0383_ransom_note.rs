pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let mut hm = HashMap::new();

        let mut count = 0;
        for ch in ransom_note.chars() {
            *hm.entry(ch).or_insert(0) += 1;
            count += 1;
        }

        for ch in magazine.chars() {
            if let Entry::Occupied(mut e) = hm.entry(ch) {
                if *e.get() > 0 {
                    *e.get_mut() -= 1;
                    count -= 1;
                    if count == 0 {
                        return true;
                    }
                }
            }
        }

        false

        /*
         * Alternate solution:
        for ch in magazine.chars() {
            *hm.entry(ch).or_insert(0) += 1;
        }

        for ch in ransom_note.chars() {
            let e = hm.entry(ch).or_insert(0);
            if *e == 0 {
                return false;
            }
            *e -= 1;
        }

        true
        */
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn ex2() {
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
    }

    #[test]
    fn ex3() {
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }
}
