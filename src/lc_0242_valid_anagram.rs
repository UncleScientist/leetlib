pub fn is_anagram(s: String, t: String) -> bool {
    let mut s = s.chars().collect::<Vec<char>>();
    let mut t = t.chars().collect::<Vec<char>>();

    s.sort();
    t.sort();

    s == t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn ex2() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }
}
