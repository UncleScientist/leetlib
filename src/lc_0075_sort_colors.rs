pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        let mut k = nums.len() - 1;

        while j <= k {
            match nums[j] {
                0 => {
                    nums.swap(i, j);
                    i += 1;
                    j += 1;
                }
                1 => {
                    j += 1;
                }
                2 => {
                    nums.swap(j, k);
                    if k < 2 {
                        break;
                    }
                    k -= 1;
                }
                _ => panic!("invalid number in input"),
            }
        }
    }

    pub fn _sort_colors(nums: &mut [i32]) {
        let mut count = [0, 0, 0];
        for n in nums.iter() {
            count[*n as usize] += 1;
        }

        let mut pos = 0;
        for (color, count) in count.iter().enumerate() {
            for i in 0..*count {
                nums[i + pos] = color as i32;
            }
            pos += count;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);
    }

    #[test]
    fn ex2() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 1, 2], nums);
    }

    #[test]
    fn fail1() {
        let mut nums = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![2], nums);
    }
}
