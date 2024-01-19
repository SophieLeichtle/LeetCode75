use std::collections::HashMap;

pub fn equal_pairs(grid:Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();


    for row in &grid {
        map.entry(row).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut pairs = 0;

    let l = grid.len();
    for i in 0..l {
        let mut col = vec![];
        for j in 0..l {
            col.push(grid[j][i]);
        }
        if let Some(elem) = map.get(&col) {
            pairs += *elem;
        }
    }

    pairs
}
