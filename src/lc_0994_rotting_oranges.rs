pub struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut minutes = 0;
        let mut next = grid;

        let mut changed = true;
        let mut finished = true;

        while changed {
            changed = false;
            finished = true;
            minutes += 1;

            let mut step = vec![vec![-1; next[0].len()]; next.len()];
            for (row, line) in next.iter().enumerate() {
                for (col, ch) in line.iter().enumerate() {
                    if *ch == 2 || *ch == 0 {
                        step[row][col] = *ch;
                    } else if row > 0 && next[row - 1][col] == 2
                        || (row < next.len() - 1) && next[row + 1][col] == 2
                        || (col > 0) && next[row][col - 1] == 2
                        || (col < next[0].len() - 1) && next[row][col + 1] == 2
                    {
                        step[row][col] = 2;
                        changed = true;
                    } else {
                        step[row][col] = 1;
                        finished = false;
                    }
                }
            }

            next = step;
        }

        if !finished {
            -1
        } else {
            minutes - 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            4,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            -1,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]])
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(0, Solution::oranges_rotting(vec![vec![0, 2]]));
    }
}
