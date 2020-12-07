use std::io::BufRead;
use std::collections::{HashMap,HashSet};
fn main1() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut bagtree : HashMap<String,Vec<String>> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        let mut ab = line
            .split("contain");
        let bag = {
            let x : Vec<&str> = ab.next().unwrap()
                .split_whitespace().collect();
            format!("{} {} bag",x[0],x[1])
        };
        let bags : Vec<String> = ab.next().unwrap()
            .trim().split(",")
            .map(|x|x.replace(".","").trim().to_string())
            .map(|x|{ if x=="no other bags" {
                "".to_string() } else {
                let y : Vec<&str> = x.split_whitespace().collect();
                format!("{} {} bag",y[1],y[2])
            }})
            .collect();

        for b in bags {
            (*bagtree.entry(b)
             .or_insert(vec![]))
                .push(bag.to_string());
        }
        if !bagtree.contains_key(&bag) {
            bagtree.insert(bag.to_string(),vec![]);
        }
    }
    println!("{:?}",bagtree);

    let mut found : HashSet<String> = HashSet::new();
    let mut stack : Vec<String> = vec!["shiny gold bag".to_string()];
    while !stack.is_empty() {
        let f = stack.pop().unwrap();
        println!("{}",f);
        let items = bagtree.get(&f).unwrap();
        if !items.is_empty() {
            found.extend(items.iter().cloned());
            stack.extend(items.iter().cloned());
        }
    }
    println!("{:?}",found.len());
}

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut bagtree : HashMap<String,Vec<(String,u64)>> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        let mut ab = line
            .split("contain");
        let bag = {
            let x : Vec<&str> = ab.next().unwrap()
                .split_whitespace().collect();
            format!("{} {} bag",x[0],x[1])
        };
        let bags : Vec<(String,u64)> = ab.next().unwrap()
            .trim().split(",")
            .map(|x|x.replace(".","").trim().to_string())
            .map(|x|{ if x=="no other bags" {
                ("".to_string(),0) } else {
                let y : Vec<&str> = x.split_whitespace().collect();
                (format!("{} {} bag",y[1],y[2]),
                 y[0].parse::<u64>().unwrap())
            }})
            .collect();

        bagtree.insert(bag.to_string(),bags);
    }
    //println!("{:?}",bagtree);

    let mut stack : Vec<(String,u64)> = vec![("shiny gold bag".to_string(),1)];
    let mut bag_count = 0;
    while !stack.is_empty() {
        let (f,c) = stack.pop().unwrap();
        if f=="" {
            continue;
        }
        let items = bagtree.get(&f).unwrap();
        //println!("{} contains {:?}" ,f,items);

        for (b,bc) in items {
            stack.push((b.to_string(),bc*c));
            bag_count+=bc*c;
        }
    }
    println!("{:?}",bag_count);
}
