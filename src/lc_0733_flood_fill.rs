pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let key = image[sr as usize][sc as usize];

    if key == color {
        return image;
    }

    let width = image[0].len() as i32;
    let height = image.len() as i32;

    image[sr as usize][sc as usize] = color;

    let mut pos_list = vec![(sr, sc)];

    while let Some(pos) = pos_list.pop() {
        let left = (pos.0, 0.max(pos.1 - 1));
        let right = (pos.0, (pos.1 + 1).min(width - 1));
        let up = (0.max(pos.0 - 1), pos.1);
        let down = ((pos.0 + 1).min(height - 1), pos.1);

        for (row, col) in [left, right, up, down] {
            if image[row as usize][col as usize] == key {
                pos_list.push((row, col));
                image[row as usize][col as usize] = color;
            }
        }
    }

    image
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let result = flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2);
        assert_eq!(result, vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]);
    }

    #[test]
    fn one_pixel() {
        let result = flood_fill(vec![vec![1, 1, 1], vec![1, 4, 0], vec![1, 0, 1]], 1, 1, 9);
        assert_eq!(result, vec![vec![1, 1, 1], vec![1, 9, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn same_color() {
        let result = flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 1);
        assert_eq!(result, vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]]);
    }
}
