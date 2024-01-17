pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        let mut len = nums.len()-1;

        loop {
            if nums[i] == 0 {
                for j in i..len {
                    nums[j] = nums[j+1];
                }
                nums[len] = 0;
                len -= 1;
            }
            if i == 0 {
                break;
            }
            i -=1;
        }
}

#[test]
fn test() {
    move_zeroes(&mut vec![0,1,0,3,12]);
}