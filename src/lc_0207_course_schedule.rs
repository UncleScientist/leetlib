pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut complete = vec![0; num_courses as usize];
        let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();

        for p in prerequisites {
            let course = p[0] as usize;
            let need = p[1] as usize;
            map.entry(course).or_default().insert(need);
            complete[course] += 1;
        }

        let mut zeroes = complete
            .iter()
            .enumerate()
            .filter(|x| *x.1 == 0)
            .map(|x| x.0)
            .collect::<Vec<_>>();

        while let Some(zero) = zeroes.pop() {
            for (k, v) in map.iter_mut() {
                if v.remove(&zero) {
                    complete[*k] -= 1;
                    if complete[*k] == 0 {
                        zeroes.push(*k);
                    }
                }
            }
        }

        complete.iter().sum::<i32>() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn ex2() {
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }

    #[test]
    fn mytest() {
        assert!(Solution::can_finish(
            3,
            vec![vec![2, 0], vec![1, 0], vec![1, 2]]
        ));
    }
}
