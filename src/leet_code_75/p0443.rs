pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut prev : char = ' ';
    let mut count = 1;

    let mut i = 0;
    while i < chars.len() {
        if chars[i] != prev {
            prev = chars[i];
            if count > 1 {
                for c in count.to_string().chars() {
                    chars.insert(i, c);
                    i += 1;
                }
            }
            i +=1;
            count = 1;
        } else {
            count += 1;
            chars.remove(i);
            
        }
    }
    if count > 1 {
        for c in count.to_string().chars() {
            chars.push(c);
        }
    }
    chars.len() as i32
}

#[test]
fn test() {
    compress(&mut vec!['a']);
    compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']);
    compress(&mut vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b']);
}