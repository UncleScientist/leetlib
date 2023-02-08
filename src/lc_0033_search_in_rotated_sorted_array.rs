pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;

        while start < end {
            let mid = (start + end) / 2;
            if nums[mid] > nums[end] {
                start = mid + 1;
            } else {
                end = mid;
            }
        }

        let pivot_point = start;
        start = 0;
        end = nums.len() - 1;

        if target >= nums[pivot_point] && target <= nums[end] {
            start = pivot_point;
        } else {
            end = pivot_point;
        }

        while start <= end {
            let mid = (start + end) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                start = mid + 1;
            } else if mid == 0 {
                return -1;
            } else {
                end = mid - 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }

    #[test]
    fn ex2() {
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    }

    #[test]
    fn ex3() {
        assert_eq!(-1, Solution::search(vec![1], 0));
    }

    #[test]
    fn fail1() {
        assert_eq!(1, Solution::search(vec![1, 3], 3));
    }
}
