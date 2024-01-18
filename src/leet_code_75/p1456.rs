pub fn max_vowels(s: String, k: i32) -> i32 {
    let mut vowels = 0;
    let k_usize = k as usize;

    let chars : Vec<char> = s.chars().collect();

    for i in 0..k_usize {
        if "aeiou".contains(chars[i]) {
            vowels += 1;
        }
    }

    if vowels == k || k_usize == chars.len(){
        return vowels;
    }

    let mut current = vowels;

    for i in k_usize..chars.len() {
        if "aeiou".contains(chars[i]) {
            current += 1;
        }
        if "aeiou".contains(chars[i - k_usize]) {
            current -= 1;
        }

        if current > vowels {
            vowels = current;
        }
    }


    vowels

}