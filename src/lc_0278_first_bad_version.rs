pub struct Solution {
    bad_vers: i32,
}

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        if self.isBadVersion(1) {
            return 1;
        }

        let mut good = 1i64;
        let mut bad = n as i64;

        loop {
            let mid = (good + bad) / 2;

            if self.isBadVersion(mid as i32) {
                bad = mid;
            } else {
                good = mid;
            }

            if good + 1 == bad {
                return bad as i32;
            }
        }
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, n: i32) -> bool {
        self.bad_vers <= n
    }

    pub fn new(bad_vers: i32) -> Self {
        Self { bad_vers }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let fbv = Solution::new(4);
        assert_eq!(fbv.first_bad_version(5), 4);
    }

    #[test]
    fn ex2() {
        let fbv = Solution::new(1);
        assert_eq!(fbv.first_bad_version(1), 1);
    }

    #[test]
    fn overflow_case() {
        let fbv = Solution::new(1702766719);
        assert_eq!(fbv.first_bad_version(2126753390), 1702766719);
    }
}
