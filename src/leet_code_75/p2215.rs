use std::{collections::{hash_map, HashMap}, hash::Hash};

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>>{
    let mut map = HashMap::new();

    for num in nums1 {
        map.entry(num).or_insert(1);
    }
    for num in nums2 {
        map.entry(num).and_modify(|n| {if *n == 1 {
            *n = 0;
        }}).or_insert(-1);
    }

    let mut d1 = vec![];
    let mut d2 = vec![];

    for (key, value) in map.iter() {
        if *value == 1 {
            d1.push(*key);
        } else if *value == -1 {
            d2.push(*key);
        }
        
    }

    vec![d1, d2]
}