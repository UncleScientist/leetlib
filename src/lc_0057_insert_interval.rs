pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut to_insert = new_interval;
        let mut pushed = false;

        for entry in intervals {
            if entry[0] > to_insert[1] {
                if !pushed {
                    result.push(to_insert.clone());
                    pushed = true;
                }
                result.push(entry);
            } else if entry[1] < to_insert[0] {
                result.push(entry)
            } else {
                to_insert[0] = to_insert[0].min(entry[0]);
                to_insert[1] = to_insert[1].max(entry[1]);
            }
        }

        if !pushed {
            result.push(to_insert);
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
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
    }
}
