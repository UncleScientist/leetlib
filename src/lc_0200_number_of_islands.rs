pub struct Solution;

impl Solution {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        use std::collections::HashSet;

        let mut map = HashSet::new();

        for (row, line) in grid.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                if *ch == '1' {
                    map.insert((row as i32, col as i32));
                }
            }
        }

        let mut island_count = 0;
        while let Some(point) = map.iter().next() {
            let mut vec = vec![*point];

            island_count += 1;
            while let Some(p) = vec.pop() {
                map.remove(&p);
                for d in Self::DIR {
                    let pos = (p.0 + d.0, p.1 + d.1);
                    if map.contains(&pos) {
                        vec.push(pos);
                    }
                }
            }
        }

        island_count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            1,
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            3,
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ])
        );
    }
}
