pub fn remove_stars(s: String) -> String {
    let mut s2 = String::new();
    for c in s.chars() {
        if c == '*' {
            s2.pop();
        } else {
            s2.push(c);
        }
    }
    s2
}