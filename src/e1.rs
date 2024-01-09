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
        let mut i = word2.len();
        while i < word1.len() {
            string.push(w1[i]);
            i += 1;
        }
    } else {
        let mut i = word1.len();
        while i < word2.len() {
            string.push(w2[i]);
            i += 1;
        }
    }

    string
}
