use std::io::BufRead;

#[derive(Debug)]
struct Seat {
    row: u32,
    col: u32,
    seatid: u32
}

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines  = reader.lines();

    let mut m  = 0;
    let mut seats : Vec<Seat> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let mut row = 0;
        for r in line.chars().take(7) {
            row = row << 1;
            if r=='B' {
                row += 1;
            }
        }
        let mut col = 0;
        for c in line.chars().skip(7) {
            col = col << 1;
            if c=='R' {
                col += 1;
            }
        }
        let seatid = row * 8 + col;
        m = m.max(seatid);
        seats.push(Seat { row,col,seatid });
    }
    println!("maxid= {:#?}",m);
    seats.sort_by_key(|k|k.seatid);

    for i in 1..seats.len() {
        if  seats[i].seatid-seats[i-1].seatid==2 {
            println!("{:#?} {:#?} {}",seats[i],seats[i-1],seats[i].seatid-1);
        }
    }
}
