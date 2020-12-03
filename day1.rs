use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut nums : Vec<i64> = Vec::new();
    for line in lines {
        let n : i64 = line.unwrap()
            .parse().unwrap();
        nums.push(n);
    }
    nums.sort();

    for i in nums.iter() {
        let j = 2020-i;
        if let Ok(r) = nums.binary_search(&j) {
            println!("{}",i*j);
        }
    }
}
