pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() == 0 {
        return true;
    }
    if t.len() == 0 {
        return false;
    }

    let mut i = 0;
    let chars : Vec<char> = s.chars().collect();

    for j in t.chars() {
        if j == chars[i] {
            i += 1;
            if i == s.len() {
                return true;
            }
        }
    }

    false
}