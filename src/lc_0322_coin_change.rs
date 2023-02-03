pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let cc = CoinChange::new(&coins);
        cc.min_coins(amount)
    }
}

struct CoinChange {
    coins: Vec<i32>,
}

impl CoinChange {
    fn new(coins: &[i32]) -> Self {
        Self {
            coins: coins.into(),
        }
    }

    fn min_coins(&self, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut change = vec![i32::MAX; (1 + amount) as usize];
        change[0] = 0;

        for i in 1..=amount {
            for coin in 0..self.coins.len() {
                if self.coins[coin] <= i {
                    let cur_count = change[(i - self.coins[coin]) as usize];
                    if cur_count != i32::MAX {
                        change[i as usize] = change[i as usize].min(cur_count + 1);
                    }
                }
            }
        }
        if change[amount as usize] == i32::MAX {
            -1
        } else {
            change[amount as usize]
        }
    }

    fn _min_coins(&self, amount: i32) -> Option<i32> {
        if amount == 0 {
            return Some(0);
        }

        let mut min_coins = None;

        for i in &self.coins {
            if *i > amount {
                continue;
            }
            if let Some(cur_count) = self._min_coins(amount - *i) {
                if let Some(min) = min_coins {
                    if cur_count + 1 < min {
                        min_coins = Some(cur_count + 1);
                    }
                } else {
                    min_coins = Some(cur_count + 1);
                }
            }
        }

        min_coins
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn failed_test() {
        assert_eq!(20, Solution::coin_change(vec![186, 419, 83, 408], 6249));
    }

    #[test]
    fn ex1() {
        assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
    }

    #[test]
    fn ex1a() {
        assert_eq!(20, Solution::coin_change(vec![1, 2, 5], 100));
    }

    #[test]
    fn ex2() {
        assert_eq!(-1, Solution::coin_change(vec![2], 3));
    }

    #[test]
    fn ex3() {
        assert_eq!(0, Solution::coin_change(vec![1], 0));
    }
}
