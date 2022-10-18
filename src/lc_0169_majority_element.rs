pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        for n in nums {
            *hm.entry(n).or_insert(0) += 1
        }
        let mut vec = hm.into_iter().collect::<Vec<(i32, i32)>>();
        vec.sort_by(|a, b| b.1.cmp(&a.1));

        vec[0].0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2)
    }
}
