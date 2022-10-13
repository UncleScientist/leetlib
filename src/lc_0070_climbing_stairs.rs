pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut fib = vec![1, 1];
        for i in fib.len()..=n as usize {
            fib.push(fib[i - 1] + fib[i - 2])
        }

        fib[n as usize]
    }
}

// 1: (1) -> 1
// 2: (2) -> 1+1, 2
// 3: (3) -> 1+1+1, 2+1, 1+2
// 4: (5) -> 1+1+1+1, 2+1+1, 1+2+1, 1+1+2, 2+2
// 5: (8) -> 1+1+1+1+1, 2+1+1+1, 1+2+1+1, 1+1+2+1, 1+1+1+2, 2+2+1, 2+1+2, 1+2+2

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::climb_stairs(6), 13);
    }
}
