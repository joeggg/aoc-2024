use std::fs;

use aoc_tools::run_solution;

fn main() {
    let (registers, program) = read_input("example.txt");
    run_solution(
        || {
            run_program(&registers, &program)
                .into_iter()
                .map(|x| x.to_string())
                .reduce(|acc, x| acc + "," + &x)
                .unwrap()
        },
        1,
    );
}

fn run_program(registers: &[u32; 3], program: &[u8]) -> Vec<u32> {
    let mut registers = *registers;
    let mut output = Vec::new();
    let mut pc = 0;
    while pc < program.len() {
        let opcode = OpCode::try_from(program[pc]).unwrap();
        let operand = program[pc + 1];
        let (out, jump) = opcode.run(operand, &mut registers);
        if let Some(val) = out {
            output.push(val);
        }
        if let Some(val) = jump {
            pc = val;
        } else {
            pc += 2;
        }
    }
    output
}

enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(format!("Invalid OpCode: {}", val)),
        }
    }
}

impl OpCode {
    fn run(&self, operand: u8, registers: &mut [u32; 3]) -> (Option<u32>, Option<usize>) {
        match self {
            Self::Adv => {
                registers[0] /= 2_u32.pow(combo_operand(operand, registers));
            }
            Self::Bxl => {
                registers[1] ^= operand as u32;
            }
            Self::Bst => {
                registers[1] = combo_operand(operand, registers) % 8;
            }
            Self::Jnz => {
                if registers[0] != 0 {
                    return (None, Some(operand as usize));
                }
            }
            Self::Bxc => {
                registers[1] ^= registers[2];
            }
            Self::Out => {
                return (Some(combo_operand(operand, registers) % 8), None);
            }
            Self::Bdv => {
                registers[1] = registers[0] / 2_u32.pow(combo_operand(operand, registers));
            }
            Self::Cdv => {
                registers[2] = registers[0] / 2_u32.pow(combo_operand(operand, registers));
            }
        }
        (None, None)
    }
}

fn combo_operand(operand: u8, registers: &[u32; 3]) -> u32 {
    if operand > 6 {
        panic!("Invalid operand: {}", operand);
    }
    if operand < 4 {
        operand as u32
    } else {
        registers[operand as usize - 4]
    }
}

fn read_input(filename: &str) -> ([u32; 3], Vec<u8>) {
    let raw = fs::read_to_string(format!("day-17/{}", filename)).unwrap();
    let (raw_regs, raw_prog) = raw.split_once("\n\n").unwrap();

    let mut regs = [0; 3];
    raw_regs.lines().enumerate().for_each(|(i, line)| {
        let (_, val) = line.trim().split_once(": ").unwrap();
        regs[i] = val.parse::<u32>().unwrap();
    });

    let (_, raw_prog) = raw_prog.trim().split_once(": ").unwrap();
    let prog = raw_prog
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    (regs, prog)
}
