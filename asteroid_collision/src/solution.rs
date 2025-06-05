pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut asteroids = asteroids;
    loop {
        let (mut x, mut crashed) = (0, false);
        while x < (asteroids.len() as i32 - 1) as usize && !asteroids.is_empty() {
            let this_ast = asteroids[x];
            let next_ast = asteroids[x + 1];
            if this_ast > 0 && next_ast < 0 {
                crashed = true;
                if this_ast.abs() == next_ast.abs() {
                    asteroids.remove(x);
                    asteroids.remove(x);
                } else if this_ast.abs() > next_ast.abs() {
                    asteroids[x] = this_ast;
                    asteroids.remove(x + 1);
                } else {
                    asteroids[x] = next_ast;
                    asteroids.remove(x + 1);
                }
            }
            x += 1;
        }
        if crashed == false {
            break;
        }
    }
    return asteroids;
}
