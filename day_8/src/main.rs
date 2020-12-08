const INPUT: &str = include_str!("../input.txt");

use std::collections::HashSet;

type Program<'a> = Vec<(&'a str, isize)>;

fn run(program: &Program) -> Result<isize, isize> {
    let mut visited: HashSet<isize> = HashSet::default();
    let mut index = 0;
    let mut accumulator = 0;

    loop {
        if !visited.insert(index) {
            break Err(accumulator);
        };

        if index >= program.len() as isize {
            break Ok(accumulator);
        }

        match program[index as usize] {
            ("nop", _) => {
                index += 1;
            }
            ("acc", n) => {
                index += 1;
                accumulator += n;
            }
            ("jmp", n) => {
                index += n;
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let program: Program = INPUT
        .split('\n')
        .map(|line| (&line[0..3], line[4..].parse::<isize>().unwrap()))
        .collect();

    let accumulator = run(&program);

    println!("{}", accumulator.unwrap_err());

    // We bruteforce by running all modified programs. The problem can be modelled as max flow (reach index n starting from
    // index 0 in the graph defined by the opcodes, augmented with edges representing opcode swaps
    // with a budget of 1 opcode swap) but this works so...
    let accumulator = program
        .iter()
        .enumerate()
        .filter_map(|(index, (instruction, _))| match *instruction {
            "jmp" => Some((index, "nop")),
            "nop" => Some((index, "jmp")),
            _ => None,
        })
        .map(|(index, instruction_name)| {
            let mut p = program.clone();
            p[index].0 = instruction_name;
            run(&p)
        })
        .find(|r| r.is_ok())
        .unwrap();

    println!("{}", accumulator.unwrap());
}
