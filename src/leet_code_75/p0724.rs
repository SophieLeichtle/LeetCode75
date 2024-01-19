pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum : i32 = nums.iter().sum();

    let mut l_sum = 0;
    for i in 0..nums.len(){
        if 2 * l_sum + nums[i] == sum {
            return i as i32;
        }
        l_sum += nums[i];
    }
    -1
}