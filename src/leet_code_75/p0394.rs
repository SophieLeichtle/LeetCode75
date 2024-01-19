pub fn decode_string(s: String) -> String {

    let mut chunks : Vec<String> = vec![String::new()];
    let mut lens : Vec<i32> = vec![];

     
    let mut chars = s.chars();



    while let Some(c) = chars.next() {

        if let Some(d) = c.to_digit(10) {
            let mut len = d;
            loop {
                let c_next = chars.next().unwrap();
                if c_next == '[' {
                    break;
                }
                len = 10 * len + c_next.to_digit(10).unwrap();
            }
            lens.push(len as i32);
            chunks.push(String::new());
        }

        
        if c.is_alphabetic(){
            let l = chunks.len() - 1;
            chunks[l].push(c);
        }

        if c == ']' {
            let len = lens.pop().unwrap();
            let chunk = chunks.pop().unwrap();
            let l = chunks.len() - 1;

            for _ in 0..len {
                chunks[l].push_str(&chunk);
            }
        }

    }

    chunks[0].clone()
}

#[test]
pub fn test() {
    println!("{}", decode_string("3[a2[c]]".to_string()));
}