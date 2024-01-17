use std::f32::INFINITY;

pub fn increasing_triplet(nums: Vec<i32>) -> bool {

        if nums.len() < 3 {
            return false;
        }

        let mut smallest = nums[0];
        let mut mid = i32::MAX;

        for i in nums {
            if i <= smallest {
                smallest = i;
            }
            else if i <= mid {
                mid = i;
            } else {
                return true
            }
        }

        false
}

#[test]
fn test () {
    println!("{}",increasing_triplet(vec![20, 100, 200, 10, 12, 5, 13]));
}