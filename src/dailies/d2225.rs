use std::collections::HashMap;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
   let mut losses = HashMap::new();

   for m in matches {
        losses.entry(m[0]).or_insert(0);
        losses.entry(m[1]).and_modify(|e| *e += 1).or_insert(1);
   }

   let mut no_losses = Vec::new();
   let mut one_loss = Vec::new();

   for loss in losses {
        if loss.1 == 0 {
            no_losses.push(loss.0);
        } else if loss.1 == 1 {
            one_loss.push(loss.0);
        }
   }

   no_losses.sort_unstable();
   one_loss.sort_unstable();

   vec![no_losses, one_loss]

}


#[test]
fn test() {
    find_winners(vec![vec![1,3], vec![2, 3]]);
}