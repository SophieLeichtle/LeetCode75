pub fn max_area(height: Vec<i32>) -> i32 {
    

    let mut i = 0;
    let mut j = height.len() - 1;

    let mut max = height[i].min(height[j]) * ((j - i) as i32);

    while j > i {

        if height[i] < height[j] {
            i += 1;

        } else {
            j -= 1;
        }

        max = max.max(height[i].min(height[j]) * ((j - i) as i32));

    }

    max
}