pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut intervals = intervals.clone();
        intervals.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut iter = intervals.iter();
        if let Some(pair) = iter.next() {
            assert_eq!(pair.len(), 2);
            let mut start = pair[0];
            let mut end = pair[1];

            while let Some(pair) = iter.next() {
                if end < pair[0] {
                    result.push(vec![start, end]);
                    start = pair[0];
                    end = pair[1];
                } else if pair[0] <= end && pair[1] >= end {
                    end = pair[1];
                }
            }
            result.push(vec![start, end]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            vec![vec![1, 5]],
            Solution::merge(vec![vec![1, 4], vec![4, 5]])
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            vec![vec![0, 4]],
            Solution::merge(vec![vec![1, 4], vec![0, 4]])
        );
    }
}
