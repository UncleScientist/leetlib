pub struct Solution;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut addr_book: HashMap<String, Vec<Address>> = HashMap::new();

        for account in accounts {
            let name = account[0].clone();
            let addr: HashSet<String> = HashSet::from_iter(account[1..].iter().cloned());

            let entry = addr_book.entry(name).or_default();
            if entry.is_empty() {
                entry.push(Address::new(addr.clone()));
            } else {
                let mut found = false;
                let mut iter = entry.iter_mut();
                while let Some(e) = iter.next() {
                    if e.try_merge(&addr) {
                        for e2 in iter.by_ref() {
                            if e.can_merge(e2) {
                                e.merge_from(e2);
                                e2.clear();
                            }
                        }
                        found = true;
                        break;
                    }
                }

                if !found {
                    entry.push(Address::new(addr.clone()));
                }
            }
        }

        let mut result = Vec::new();
        for (key, val) in addr_book {
            for entry in val.iter().filter(|v| !v.is_empty()) {
                let mut address = vec![key.clone()];
                let emails = entry.get_list();
                address.extend(emails);
                result.push(address);
            }
        }

        result
    }
}

#[derive(Debug)]
struct Address {
    addr: HashSet<String>,
}

impl Address {
    fn new(addr: HashSet<String>) -> Self {
        Self { addr }
    }

    fn is_empty(&self) -> bool {
        self.addr.is_empty()
    }

    fn can_merge(&self, other: &Address) -> bool {
        self.addr.intersection(&other.addr).count() != 0
    }

    fn merge_from(&mut self, other: &Address) {
        self.addr = self.addr.union(&other.addr).cloned().collect();
    }

    fn clear(&mut self) {
        self.addr.clear();
    }

    fn get_list(&self) -> Vec<String> {
        let mut list = self.addr.iter().cloned().collect::<Vec<String>>();
        list.sort();
        list
    }

    fn try_merge(&mut self, other: &HashSet<String>) -> bool {
        if self.addr.intersection(other).count() != 0 {
            self.addr = self.addr.union(other).cloned().collect();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::vecs_equal;

    #[test]
    fn ex1() {
        let accounts = vec![
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john_newyork@mail.com".to_string(),
            ],
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john00@mail.com".to_string(),
            ],
            vec!["Mary".to_string(), "mary@mail.com".to_string()],
            vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        ];
        let output = vec![
            vec![
                "John".to_string(),
                "john00@mail.com".to_string(),
                "john_newyork@mail.com".to_string(),
                "johnsmith@mail.com".to_string(),
            ],
            vec!["Mary".to_string(), "mary@mail.com".to_string()],
            vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        ];
        let result = Solution::accounts_merge(accounts);
        assert!(vecs_equal(&result, &output));
    }

    #[test]
    fn fail1() {
        let accounts = vec![
            vec![
                "David".to_string(),
                "David0@m.co".to_string(),
                "David1@m.co".to_string(),
            ],
            vec![
                "David".to_string(),
                "David3@m.co".to_string(),
                "David4@m.co".to_string(),
            ],
            vec![
                "David".to_string(),
                "David4@m.co".to_string(),
                "David5@m.co".to_string(),
            ],
            vec![
                "David".to_string(),
                "David2@m.co".to_string(),
                "David3@m.co".to_string(),
            ],
            vec![
                "David".to_string(),
                "David1@m.co".to_string(),
                "David2@m.co".to_string(),
            ],
        ];
        let output = vec![vec![
            "David".to_string(),
            "David0@m.co".to_string(),
            "David1@m.co".to_string(),
            "David2@m.co".to_string(),
            "David3@m.co".to_string(),
            "David4@m.co".to_string(),
            "David5@m.co".to_string(),
        ]];
        let result = Solution::accounts_merge(accounts);
        assert!(vecs_equal(&result, &output));
    }
}
