pub fn reverse_vowels(s: String) -> String {
    let vowels = vec!['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];

    let filtered : Vec<usize> = s.chars().enumerate().filter_map(|(i, c)| {
        if vowels.contains(&c) {
            return Some(i);
        }
        None
    }).collect();

    if filtered.len() == 0 {
        return s;
    }

    let mut string : Vec<_> = s.chars().collect();
    let mut i = 0;
    let mut j = filtered.len() - 1;

    while i < j {
        string.swap(filtered[i], filtered[j]);
        i += 1;
        j -= 1;
    }

    string.iter().collect()
}