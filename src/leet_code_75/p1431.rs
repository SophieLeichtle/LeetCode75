pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.iter().max().unwrap() - extra_candies;
    
    let mut vec = Vec::new();
    for i in 0..candies.len() {
        if candies[i] >= max {
            vec.push(true);
        }else {
            vec.push(false);
        }
    }

    vec
}