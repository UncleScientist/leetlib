pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::hash_map::Entry;
        use std::collections::{HashMap, HashSet};
        let mut result = HashSet::new();

        let mut hm = HashMap::new();
        for (idx, val) in nums.iter().enumerate() {
            hm.insert(*val, idx);
        }

        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                let val = -(nums[i] + nums[j]);
                if let Entry::Occupied(entry) = hm.entry(val) {
                    let idx = *entry.get();
                    if i != idx && j != idx {
                        let mut v = vec![nums[i], nums[j], val];
                        v.sort();
                        result.insert(v);
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vecs_equal;

    #[test]
    fn ex1() {
        assert!(vecs_equal(
            &Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            &[vec![-1, -1, 2], vec![-1, 0, 1]]
        ));
    }

    #[test]
    fn ex2() {
        assert!(vecs_equal(&Solution::three_sum(vec![0, 1, 1]), &[]));
    }

    #[test]
    fn ex3() {
        assert!(vecs_equal(
            &Solution::three_sum(vec![0, 0, 0]),
            &[vec![0, 0, 0]]
        ));
    }
}
