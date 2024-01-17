pub fn reverse_words(s: String) -> String {
    s.split_whitespace().map(|s| s.to_string()).reduce(|acc, e| e + " " + &acc ).unwrap()
}