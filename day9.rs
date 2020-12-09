use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    let mut numbers : Vec<u64> = lines
        .map(|x|x.unwrap().parse().unwrap())
        .collect();
    let preamble = 25;
    let mut invalid_number = 0;
    for i in (preamble)..numbers.len() {
        let lower =  i-preamble;
        let upper =  i;
        let mut found = false;
        for j in lower..upper {
            for k in lower..upper {
                if i!=k && numbers[i]==numbers[j]+numbers[k] {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            println!("rulebreaker={}",numbers[i]);
            invalid_number = numbers[i];
            break;
        }
    }

    // complete search
    for p in 2..20 {
        for i in p..numbers.len() {
            let lower =  i-p;
            let upper =  i;
            let mut sum = 0;
            for j in lower..upper {
                sum+=numbers[j];
            }
            if sum == invalid_number {
                let mut min = numbers[lower];
                let mut max = numbers[lower];
                for j in lower..upper {
                    if numbers[j]<min { min = numbers[j]; }
                    if numbers[j]>max { max = numbers[j]; }
                }
                println!("p={} min={} max={} sum={}",p,min,max,min+max);
                break;
            }
        }
    }
}
