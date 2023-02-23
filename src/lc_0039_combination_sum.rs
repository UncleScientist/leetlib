use std::iter;

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let c = Combinator::new(&candidates, target);
        c.collect()
    }
}

struct Combinator {
    nums: Vec<i32>,
    target: i32,
    mult: Vec<i32>,
}

impl Combinator {
    fn new(candidates: &[i32], target: i32) -> Self {
        let mut nums = candidates.to_vec();
        let len = nums.len();
        nums.sort();

        Self {
            nums,
            target,
            mult: vec![0; len],
        }
    }

    fn value(&self) -> i32 {
        self.nums
            .iter()
            .zip(self.mult.iter())
            .map(|(a, b)| *a * *b)
            .sum()
    }

    fn step(&mut self) -> bool {
        let mut idx = 0;

        while idx < self.nums.len() {
            self.mult[idx] += 1;
            if self.value() <= self.target {
                return true;
            }
            self.mult[idx] = 0;
            idx += 1;
        }

        false
    }
}

impl Iterator for Combinator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if !self.step() {
            return None;
        }

        let mut loop_limit = 1000;
        while self.value() != self.target {
            if !self.step() {
                return None;
            }
            loop_limit -= 1;
            if loop_limit == 0 {
                panic!("possible infinite loop detected");
            }
        }

        let mut result = Vec::new();
        for (idx, val) in self.mult.iter().enumerate() {
            result.extend(iter::repeat(self.nums[idx]).take(*val as usize));
        }

        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            vec![vec![2, 2, 3], vec![7]],
            Solution::combination_sum(vec![2, 3, 6, 7], 7)
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            Solution::combination_sum(vec![2, 3, 5], 8)
        );
    }

    #[test]
    fn ex3() {
        assert!(Solution::combination_sum(vec![2], 1).is_empty());
    }

    #[test]
    fn combination_test() {
        let mut c = Combinator::new(&[2, 3, 5, 7], 7);
        assert!(c.step());
        assert_eq!(vec![1, 0, 0, 0], c.mult);
        assert!(c.step());
        assert_eq!(vec![2, 0, 0, 0], c.mult);
        assert!(c.step());
        assert_eq!(vec![3, 0, 0, 0], c.mult);
        assert!(c.step());
        assert_eq!(vec![0, 1, 0, 0], c.mult);
        assert!(c.step());
        assert_eq!(vec![1, 1, 0, 0], c.mult);
    }
}
