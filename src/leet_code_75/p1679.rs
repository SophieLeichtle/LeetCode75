pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut ops = 0;

    let mut copy = nums.clone();
    copy.sort_unstable();

    let mut l = 0;
    let mut r = copy.len() - 1;

    while r > l {

        if copy[r] + copy[l] == k {
            ops += 1;
            l += 1;
            r -= 1;
        } else if copy[r] + copy[l] < k {
            l += 1;
        } else {
            r -= 1;
        }
    }

    ops

}