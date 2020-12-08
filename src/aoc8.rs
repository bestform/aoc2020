use crate::rd;

pub(crate) fn run() {
    let contents = rd!("src/input/08a.txt");

    let input = instructions_from_string(contents);

    let p = Proc{
        acc: 0,
        visited: vec![],
        instruction_pointer: 0,
        program: input.clone()
    };

    let res = run_proc(p);

    match res {
        ProcResult::Err(proc, _) => {
            println!("Errored wit acc at {}", proc.acc);
        }
        ProcResult::Done(proc) => {
            println!("Ended wit acc at {}", proc.acc);
        }
    }

    for mutation in mutate(input) {
        let p = Proc{
            acc: 0,
            visited: vec![],
            instruction_pointer: 0,
            program: mutation
        };

        let res = run_proc(p);
        match res {
            ProcResult::Err(_, _) => {
                // do nothing
            }
            ProcResult::Done(proc) => {
                println!("Found working mutation! Acc: {}", proc.acc);
            }
        }
    }
}

struct Proc {
    acc: i32,
    visited: Vec<i32>,
    instruction_pointer: i32,
    program: Vec<Instruction>,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum ProcResult {
    Err(Proc, String),
    Done(Proc)
}

fn run_proc(mut p: Proc) -> ProcResult {

    loop {
        if p.visited.contains(&p.instruction_pointer) {
            return ProcResult::Err(p, "Loop".to_string());
        }
        let i:Option<&Instruction> = p.program.get(p.instruction_pointer as usize);
        match i {
            None => {
                return ProcResult::Done(p);
            }
            Some(instruction) => {
                p.visited.push(p.instruction_pointer);
                match instruction {
                    Instruction::Acc(add) => {
                        p.acc += add;
                    }
                    Instruction::Jmp(jmp) => {
                        p.instruction_pointer += jmp;
                        continue;
                    }
                    Instruction::Nop(_) => {

                    }
                }
                p.instruction_pointer += 1;
            }
        }
    }
}

fn instructions_from_string(input: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<_>>();
        let num = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "acc" => instructions.push(Instruction::Acc(num)),
            "jmp" => instructions.push(Instruction::Jmp(num)),
            "nop" => instructions.push(Instruction::Nop(num)),
            _ => panic!()
        }
    }

    return instructions;
}

fn mutate(input: Vec<Instruction>) -> Vec<Vec<Instruction>> {
    let mut mutations: Vec<Vec<Instruction>> = vec![];
    mutations.push(input.clone());

    for (idx, i) in input.clone().into_iter().enumerate() {
        match i {
            Instruction::Acc(_) => {

            }
            Instruction::Jmp(inp) => {
                let mut mutation:Vec<Instruction> = input.clone();
                mutation[idx] = Instruction::Nop(inp);
                mutations.push(mutation);
            }
            Instruction::Nop(inp) => {
                let mut mutation:Vec<Instruction> = input.clone();
                mutation[idx] = Instruction::Jmp(inp);
                mutations.push(mutation);
            }
        }
    }

    return mutations;
}

#[test]
fn test_mutate() {
    let input = vec![
        Instruction::Nop(0),
        Instruction::Acc(1),
        Instruction::Jmp(4),
        Instruction::Acc(3),
        Instruction::Jmp(-3),
        Instruction::Acc(-99),
        Instruction::Acc(1),
        Instruction::Jmp(-4),
        Instruction::Acc(6),
    ];
    assert_eq!(mutate(input).len(), 5);
}

#[test]
fn test_parse() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6".to_string();
    let expected = vec![
        Instruction::Nop(0),
        Instruction::Acc(1),
        Instruction::Jmp(4),
        Instruction::Acc(3),
        Instruction::Jmp(-3),
        Instruction::Acc(-99),
        Instruction::Acc(1),
        Instruction::Jmp(-4),
        Instruction::Acc(6),
    ];
    let actual = instructions_from_string(input);
    assert_eq!(actual.len(), expected.len());
}

#[test]
fn example() {
    let p = Proc{
        acc: 0,
        visited: vec![],
        instruction_pointer: 0,
        program: vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ]
    };

    let res = run_proc(p);
    match res {
        ProcResult::Err(p, err) => {
            assert_eq!(p.acc, 5);
            assert_eq!(err, "Loop".to_string());
        }
        ProcResult::Done(_) => {
            panic!("Expected Error")
        }
    }

}