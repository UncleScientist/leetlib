pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut dist = points
            .into_iter()
            .map(|p| (p.clone(), (p[0] * p[0] + p[1] * p[1])))
            .collect::<Vec<(Vec<i32>, i32)>>();

        dist.sort_by(|a, b| a.1.cmp(&b.1));

        dist.iter()
            .take(k as usize)
            .map(|e| e.0.clone())
            .collect::<Vec<Vec<i32>>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2], vec![2, -2]], 2),
            vec![vec![-2, 2], vec![2, -2]]
        );
    }
}
