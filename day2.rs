use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut valid = 0;
    while let Some(Ok(line)) = lines.next() {
        let abc : Vec<String> = line
            .split_whitespace()
            .map(|x|x.to_string())
            .collect();
        let loup : Vec<u32>  = abc[0]
            .split("-")
            .map(|x|x.parse().unwrap())
            .collect();
        let lower = loup[0] as usize;
        let upper = loup[1] as usize;
        let c = abc[1].chars().nth(0).unwrap();
        let pw = abc[2].to_string();
        let mut cnt = 0;
        for (i,ch) in pw.chars().enumerate() {
            if i+1==lower && c==ch {
                cnt+=1;
            }
            if i+1==upper && c==ch {
                cnt+=1;
            }
        }
        if cnt==1 {
            valid+=1;
        }
    }
    println!("{}",valid);
}
