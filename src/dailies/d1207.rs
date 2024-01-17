use std::collections::HashMap;

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut map = HashMap::new();

    for x in arr {
        map.entry(x).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut occurrences : Vec<&i32> = map.values().collect();
    occurrences.sort_unstable();

    for i in 0..occurrences.len() - 1 {
        if occurrences[i] == occurrences[i+1] {
            return false;
        }
    }
    true
}