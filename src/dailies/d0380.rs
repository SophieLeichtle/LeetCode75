use std::collections::HashMap;
use rand::seq::SliceRandom;


struct RandomizedSet {
    set: HashMap<i32, usize>,
    vec: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet {
            set: HashMap::new(),
            vec: Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if !self.set.contains_key(&val) {
            self.vec.push(val);
            self.set.insert(val, self.vec.len() - 1);
            return true;
        }
        false
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if self.set.contains_key(&val) {
            let index = self.set.remove(&val).unwrap();
            self.vec.swap_remove(index);
            if index != self.vec.len(){
               self.set.insert(self.vec[index], index); 
            }
            
            return true;
        }

        false
    }
    
    fn get_random(&self) -> i32 {
        *self.vec.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[test]
fn test() {
    let mut rset = RandomizedSet::new();

    rset.insert(3);
    rset.insert(3);
    rset.get_random();
    rset.get_random();
    rset.insert(1);
    rset.remove(3);
    rset.get_random();
    rset.get_random();
    rset.insert(0);
    rset.remove(0);
}