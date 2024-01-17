use std::iter::zip;

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut string = String::new();

    let w1 : Vec<char> = word1.chars().collect();
    let w2 : Vec<char> = word2.chars().collect();
    
    for (x, y) in zip(w1.clone(), w2.clone()) {
        string.push(x);
        string.push(y);
    }

    if word1.len() > word2.len() {
        string.push_str(&word1[word2.len()..]);
        
    } else {
        string.push_str(&word2[word1.len()..]);
    }

    string
}
