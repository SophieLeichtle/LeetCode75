pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    if !nums.contains(&0) {
        return nums.len() as i32 - 1;
    }

    let splits : Vec<usize> = nums.split(|e| *e == 0).map(|e| e.len()).collect();
    let mut max = 0;
    for i in 1..splits.len() {
        max = max.max(splits[i] + splits[i-1]);
    }
    max as i32
    }