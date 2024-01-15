use std::collections::HashMap;

pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut map = HashMap::new();
        for c in word1.chars() {
            map.entry(c).and_modify(|i| *i += 1).or_insert(1);
        }

        let mut k1 = Vec::new();
        let mut v1 = Vec::new();

        for (k, v) in map.drain() {
            k1.push(k);
            v1.push(v);
        }

        k1.sort_unstable();
        v1.sort_unstable();

        for c in word2.chars() {
            map.entry(c).and_modify(|i| *i += 1).or_insert(1);
        }

        let mut k2 = Vec::new();
        let mut v2 = Vec::new();

        for (k, v) in map.drain() {
            k2.push(k);
            v2.push(v);
        }

        k2.sort_unstable();
        v2.sort_unstable();

        println!("{:?}", k1);
        println!("{:?}", k2);

        println!("{:?}", v1);
        println!("{:?}", v2);

        if k1 != k2 { 
            return false;
        }

        if v1 != v2 {
            return false;
        }

        true
}



#[test]
fn test() {
    close_strings("cabbba".to_string(), "abbccc".to_string());
}
