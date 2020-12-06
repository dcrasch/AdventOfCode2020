use std::io::BufRead;
use std::collections::HashSet;

fn anyone_yes(answers: &Vec<String>) -> usize {
    let a : HashSet<char> = answers.into_iter().map(|x|x.chars()).flatten().collect();
    a.len()
}

fn everyone_yes(answers: &Vec<String>) -> usize {
    let mut everyone_answers  : HashSet<char> = answers[0].chars().collect();
    for person_answers in answers.iter().skip(1) {
        let h2 : HashSet<char> = person_answers.chars().collect();
        everyone_answers = everyone_answers.intersection(&h2).cloned().collect();
    }
    everyone_answers.len()
}

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines  = reader.lines();

    let mut answers : Vec<String> = Vec::new();
    let mut anyone_count = 0_usize;
    let mut everyone_count = 0_usize;
    while let Some(Ok(line)) = lines.next() {
        if line=="" {
            anyone_count += anyone_yes(&answers);
            everyone_count += everyone_yes(&answers);
            answers = Vec::new();
        }
        else  {
            answers.push(line);
        }
    }
    anyone_count += anyone_yes(&answers);
    everyone_count += everyone_yes(&answers);
    println!("anyone = {}, everyone = {}",anyone_count,everyone_count);
}
