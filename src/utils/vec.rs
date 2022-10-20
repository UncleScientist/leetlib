use std::collections::HashSet;

pub fn vecs_equal<T: std::cmp::PartialEq<T>>(v1: &[T], v2: &[T]) -> bool {
    if v1.len() != v2.len() {
        return false;
    }

    let mut hs = HashSet::<usize>::from_iter(0..v2.len());
    for val in v1.iter() {
        let mut loc = None;
        for &h in hs.iter() {
            if v2[h] == *val {
                loc = Some(h);
                break;
            }
        }

        if let Some(loc) = loc {
            hs.remove(&loc);
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare_empty() {
        assert!(vecs_equal::<i32>(&[], &[]));
    }

    #[test]
    fn compare_singles() {
        assert!(vecs_equal(&[1], &[1]));
    }

    #[test]
    fn compare_one_unequal() {
        assert!(!vecs_equal(&[1], &[2]));
    }

    #[test]
    fn compare_out_of_order() {
        assert!(vecs_equal(&[1, 2, 3, 4, 5], &[5, 2, 4, 3, 1]));
    }
}
