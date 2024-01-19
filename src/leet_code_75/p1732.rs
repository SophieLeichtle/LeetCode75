pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut current = 0;

    for g in gain {
        current += g;
        if current > max {
            max = current;
        }
    }

    max
}