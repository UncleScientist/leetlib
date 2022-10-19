pub struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let width = mat[0].len();
        let height = mat.len();
        let mut output = vec![vec![0; width]; height];
        let max = (width + height) as i32;

        let mut hs = HashSet::new();

        for row in 0..height {
            for col in 0..width {
                if mat[row][col] == 0 {
                    output[row][col] = 0;
                } else {
                    output[row][col] = max;
                    hs.insert((row, col));
                }
            }
        }

        while !hs.is_empty() {
            let entries = hs.drain().collect::<Vec<(usize, usize)>>();

            for (row, col) in entries {
                let up = if row == 0 { None } else { Some((row - 1, col)) };
                let down = if row + 1 == height {
                    None
                } else {
                    Some((row + 1, col))
                };
                let left = if col == 0 { None } else { Some((row, col - 1)) };
                let right = if col + 1 == width {
                    None
                } else {
                    Some((row, col + 1))
                };

                let valid = [up, down, left, right]
                    .iter()
                    .filter_map(|&x| x)
                    .collect::<Vec<(usize, usize)>>();
                let min = 1 + valid.iter().fold(max, |a, b| a.min(output[b.0][b.1]));

                if min < output[row][col] {
                    output[row][col] = min;
                    for dir in valid {
                        hs.insert(dir);
                    }
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}
