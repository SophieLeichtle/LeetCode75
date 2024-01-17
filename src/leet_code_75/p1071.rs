pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let string = String::new();

    let gcd = euclid(str1.len(), str2.len());
    let factors = brute_force_factor(gcd);

    for f in factors {
        let slice = &str1[..f];

        let mult = str1.len() / f;
        let mut str = String::new();
        for _ in 0..mult {
            str.push_str(slice);
        }
        if str == str1 {
            let mult2 = str2.len()/f;
            str = String::new();
            for _ in 0..mult2 {
                str.push_str(slice);
            }
            if str == str2 {
                return slice.to_string()
            }
        }
    }

    string
}

pub fn brute_force_factor(input: usize) -> Vec<usize> {
    let mut out = vec![];

    for p in 1..input +1  {
        if input % p == 0 {
            out.push(p);
        }
    }
    out.reverse();
    out
}

pub fn euclid(a: usize, b: usize) -> usize {
    let mut ac = a;
    let mut bc = b;
    while bc != 0 {
        let t = bc;
        bc = ac%bc;
        ac = t;
    }
    ac
}