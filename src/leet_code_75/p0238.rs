pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut out = vec![1; nums.len()];

    let mut left = 1;
    let mut right = 1;

    let mut l = 0;
    let mut r = nums.len() - 1;

    loop {
        out[l] *= left;
        out[r] *= right;

        left *= nums[l];
        right *= nums[r];

        if r == 0 {
            break;
        }
        
        l += 1;
        r -= 1;
    }

    out
}

#[test]
fn test(){
    product_except_self(vec![1,2,3,4]);
}