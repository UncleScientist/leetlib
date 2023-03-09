use std::collections::HashMap;

#[derive(Default)]
pub struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store
            .entry(key)
            .or_insert_with(Vec::new)
            .push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(vec) = self.store.get(&key) {
            match vec.binary_search_by(|item| item.0.cmp(&timestamp)) {
                Ok(found) => vec[found].1.clone(),
                Err(not_found) if not_found > 0 => vec[not_found - 1].1.clone(),
                Err(_) => "".to_string(),
            }
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut time_map = TimeMap::new();

        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!("bar".to_string(), time_map.get("foo".to_string(), 1));
        assert_eq!("bar".to_string(), time_map.get("foo".to_string(), 3));

        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!("bar".to_string(), time_map.get("foo".to_string(), 3));
        assert_eq!("bar2".to_string(), time_map.get("foo".to_string(), 4));
        assert_eq!("bar2".to_string(), time_map.get("foo".to_string(), 5));
    }
}
