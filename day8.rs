use std::io::BufRead;

#[derive(Debug)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct Instruction(InstructionKind, i32);

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut code: Vec<Instruction> = lines
        .map(|x| {
            let ib: Vec<String> = x
                .unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();
            let i = ib[0].clone();
            let a = ib[1].parse().unwrap();
            match i.as_str() {
                "acc" => Instruction(InstructionKind::Acc, a),
                "jmp" => Instruction(InstructionKind::Jmp, a),
                _ => Instruction(InstructionKind::Nop, a),
            }
        })
        .collect();

    let icount : usize = code.len();

    for i in 0..icount {
        match code[i] {
            Instruction(InstructionKind::Jmp, a) => {
                code[i] = Instruction(InstructionKind::Nop,a);
            },
            Instruction(InstructionKind::Nop, a) => {
                code[i] = Instruction(InstructionKind::Jmp,a);
            },
            _ => {}
        }
        let mut run: Vec<bool> = vec![false; code.len()];
        let mut pc = 0_i32;
        let mut acc = 0;

        loop {
            run[pc as usize] = true;
            match code[pc as usize] {
                Instruction(InstructionKind::Acc, a) => {
                    acc += a;
                    pc += 1;
                }
                Instruction(InstructionKind::Jmp, a) => {
                    pc += a;
                }
                _ => {
                    pc += 1;
                }
            }
            if pc as usize == icount || run[pc as usize] {
                break;
            }
        }
        if pc as usize == icount {
            println!("acc {}", acc);
        }
        // switch back
        match code[i] {
            Instruction(InstructionKind::Jmp, a) => {
                code[i] = Instruction(InstructionKind::Nop,a);
            },
            Instruction(InstructionKind::Nop, a) => {
                code[i] = Instruction(InstructionKind::Jmp,a);
            },
            _ => {}
        }
    }
}
