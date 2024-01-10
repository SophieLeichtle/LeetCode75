pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {

    let filtered = filter(flowerbed);
    filtered.iter().sum::<i32>() >= n
    
}

pub fn filter(mut vec: Vec<i32>) -> Vec<i32> {
    let mut filtered = Vec::new();
    let mut prev = false;

    vec.insert(0, 0);
    vec.push(0);

    vec.windows(3).for_each(|slice| {
        if prev || slice.contains(&1) {
            prev = false;
            filtered.push(0);
        } else {
            filtered.push(1);
            prev = true;
        }
    });
    
    filtered
}