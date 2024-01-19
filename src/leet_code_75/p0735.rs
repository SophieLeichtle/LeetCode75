pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut post : Vec<i32> = vec![];

    let mut i = 0;
    while i < asteroids.len() {
        let asteroid = asteroids[i];
        if post.is_empty() || !(post[post.len() - 1].signum() == 1 && asteroid.signum() == -1)  {
            post.push(asteroid);
            i += 1;
        } else {
            let prev = post[post.len()-1];
            if prev.abs() < asteroid.abs() {
                post.pop();
            } else if prev.abs() == asteroid.abs() {
                post.pop();
                i += 1;
            } else {
                i += 1;
            }
        }
    }

    post
}