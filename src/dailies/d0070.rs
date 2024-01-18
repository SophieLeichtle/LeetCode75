pub fn climb_stairs(n: i32) -> i32 {
    let mut memo = vec![-1; (n+1) as usize];
    count_recursive(n, &mut memo)    
}

pub fn count_recursive(n: i32, memo: &mut Vec<i32>) -> i32 {
    if n <= 2 {
        return n;
    }
    if memo[n as usize] != -1 {
        return memo[n as usize];
    }

    memo[n as usize] = count_recursive(n - 1, memo) + count_recursive(n - 2, memo);
    
    memo[n as usize]
}

#[test]
fn test() {
    println!("{}",climb_stairs(2));
    println!("{}",climb_stairs(45));
}