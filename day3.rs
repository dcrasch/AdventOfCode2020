use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let m: Vec<Vec<bool>> = lines
        .map(|x| x.unwrap().chars().map(|y| y == '#').collect())
        .collect();

    let height = m.len();
    let width =  m[0].len();

    let steps = vec![(1,1),(1,3),(1,5),(1,7),(2,1)];
    let mut res = 1;
    for step in steps {
        let mut tree = 0;
        let mut col = 0;
        for row in (0..height).step_by(step.0) {
            if m[row][col%width] {
                tree+=1;
            }
            col+=step.1;
        }
        println!("{:?} {}",step,tree);
        res*=tree;
    }
    println!("{}",res);
}
