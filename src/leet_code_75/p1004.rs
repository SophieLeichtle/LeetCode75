pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut zeros = 0;
        let mut max = 0;

        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            if nums[r] == 1 {
                r += 1;
                max = max.max(r - l);
                continue;
            }

            if zeros < k {
                r += 1;
                zeros += 1;
                max = max.max(r - l);
                continue;
            }

            if nums[l] == 0 {
                zeros -= 1
            }
            l += 1;
            
            
        }

        max as i32
}

#[test]
fn test() {
    longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2);
}