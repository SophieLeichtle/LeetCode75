struct RecentCounter {
    queue: std::collections::VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            queue: std::collections::VecDeque::new(),
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        while self.queue[0] < t - 3000 {
            self.queue.pop_front();
        }
        self.queue.len() as i32
    }
}
