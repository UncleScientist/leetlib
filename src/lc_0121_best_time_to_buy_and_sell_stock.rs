pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut max = 0.max(prices[1] - prices[0]);
    let mut min = prices[0];

    for price in prices.iter().skip(1) {
        max = max.max(*price - min);
        min = min.min(*price);
    }

    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn ex2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
