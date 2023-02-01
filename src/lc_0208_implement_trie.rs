use std::{
    collections::{hash_map::Entry, HashMap},
    str::Chars,
};

#[derive(Default, Debug)]
pub struct Trie {
    trielist: Vec<Self>,
    children: HashMap<char, usize>, // map a char to an index in trielist
    word: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    fn char_insert(&mut self, mut iter: Chars) {
        if let Some(ch) = iter.next() {
            /*
            let e = self.children.entry(ch).or_insert_with(|| {
                let num = self.trielist.len();
                self.trielist.push(Trie::new());
                num
            });
            */
            match self.children.entry(ch) {
                Entry::Occupied(o) => self.trielist[*o.get()].char_insert(iter),
                Entry::Vacant(v) => {
                    let num = self.trielist.len();
                    self.trielist.push(Trie::new());
                    v.insert(num);
                    self.trielist[num].char_insert(iter);
                }
            }
        } else {
            self.word = true;
        }
    }

    fn char_search(&self, mut iter: Chars, full: bool) -> bool {
        if let Some(ch) = iter.next() {
            if let Some(trie) = self.children.get(&ch) {
                self.trielist[*trie].char_search(iter, full)
            } else {
                false
            }
        } else if full && self.word {
            true
        } else {
            !full
        }
    }

    pub fn insert(&mut self, word: String) {
        self.char_insert(word.chars());
    }

    pub fn search(&self, word: String) -> bool {
        self.char_search(word.chars(), true)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.char_search(prefix.chars(), false)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut trie = Trie::new();
        trie.insert("apple".into());
        assert!(trie.search("apple".into())); // return True
        assert!(!trie.search("app".into())); // return False
        assert!(trie.starts_with("app".into())); // return True
        trie.insert("app".into());
        assert!(trie.search("app".into())); // return True
    }
}

/*
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
