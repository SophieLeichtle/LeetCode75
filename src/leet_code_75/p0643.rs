pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    
    
    let k_usize = k as usize;
    let mut avg: i32 = nums[0..k_usize].iter().sum();
    
    let mut current = avg;


    for i in k_usize..nums.len() {
        current = current - nums[i - k_usize] + nums[i];
        if current > avg {
            avg = current;
        }
    }

    avg as f64 / k as f64
    
}