pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    if matrix.len() == 1 {
        return matrix[0][0];
    }

    let mut sums = vec![matrix[0].clone()];

    let row_len = sums[0].len();

    for i in 1..matrix.len() {
        sums.push(vec![0;row_len]);
        for j in 0..row_len {
            if j == 0 {
                sums[i][j] = matrix[i][0] + sums[i-1][0].min(sums[i-1][1]);
            } else if j == row_len - 1 {
                sums[i][j] = matrix[i][row_len - 1] + sums[i-1][row_len - 2].min(sums[i-1][row_len - 1]);
            } else {
                sums[i][j] = matrix[i][j] + sums[i-1][j - 1].min(sums[i-1][j].min(sums[i-1][j+1]));
            }
        }
    }

    let mut min = i32::MAX;
    for s in &sums[row_len-1] {
        if *s < min {
            min = *s;
        }
    }
    min
}

#[test]
pub fn test() {
    min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]);
}