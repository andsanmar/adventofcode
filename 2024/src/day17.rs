use std::char;

use aoc::input_file;

type Lit = usize;

#[derive(Debug)]
enum Combo {
    Lit(Lit),
    Reg(usize),
}

#[derive(Debug)]
enum Inst {
    ADV(Combo),
    BXL(Lit),
    BST(Combo),
    JNZ(Lit),
    BXC,
    OUT(Combo),
    BDV(Combo),
    CDV(Combo),
}

#[derive(Debug)]
struct Data {
    registers: [usize; 3],
    instructions: Vec<Inst>,
    pc: usize,
    output: Vec<Lit>,
    text_program: Vec<Lit>,
}

fn parse_combo(c: char) -> Combo {
    match c {
        '0'..='3' => Combo::Lit(parse_lit(c)),
        '4'..='7' => Combo::Reg(parse_reg(c)),
        _ => panic!("Invalid operand"),
    }
}

fn parse_reg(c: char) -> usize {
    match c {
        '4' => 0,
        '5' => 1,
        '6' => 2,
        _ => panic!("Invalid register"),
    }
}

fn parse_lit(c: char) -> Lit {
    c.to_digit(8).unwrap() as Lit
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut registers = [0, 0, 0];
        let mut instructions = Vec::new();
        let mut it = input.split("\n\n");
        let mut reg_index = 0;
        for line in it.next().unwrap().lines() {
            registers[reg_index] = line.split_whitespace().last().unwrap().parse().unwrap();
            reg_index += 1;
        }

        let text_program = &it.next().unwrap()[9..];
        for chars in text_program.chars().collect::<Vec<char>>().chunks(4) {
            match chars {
                [ins, _, op, _] | [ins, _, op] => match ins {
                    '0' => instructions.push(Inst::ADV(parse_combo(*op))),
                    '1' => instructions.push(Inst::BXL(parse_lit(*op))),
                    '2' => instructions.push(Inst::BST(parse_combo(*op))),
                    '3' => instructions.push(Inst::JNZ(parse_lit(*op))),
                    '4' => instructions.push(Inst::BXC),
                    '5' => instructions.push(Inst::OUT(parse_combo(*op))),
                    '6' => instructions.push(Inst::BDV(parse_combo(*op))),
                    '7' => instructions.push(Inst::CDV(parse_combo(*op))),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(Data {
            registers,
            instructions,
            pc: 0,
            output: Vec::new(),
            text_program: text_program
                .split(",")
                .map(|s| parse_lit(s.chars().next().unwrap()))
                .collect(),
        })
    }
}

impl Data {
    fn combo_value(&self, combo: &Combo) -> Lit {
        match combo {
            Combo::Lit(lit) => *lit,
            Combo::Reg(reg) => self.registers[*reg],
        }
    }

    fn execute_ins(&mut self) {
        use Inst::*;

        match &self.instructions[self.pc] {
            ADV(op) => self.registers[0] /= 1 << self.combo_value(op),
            BXL(op) => self.registers[1] ^= *op,
            BST(op) => self.registers[1] = self.combo_value(op) % 8,
            JNZ(op) => {
                if self.registers[0] != 0 {
                    self.pc = *op - 1;
                }
            }
            BXC => self.registers[1] ^= self.registers[2],
            OUT(op) => self.output.push(self.combo_value(op) % 8),
            BDV(op) => self.registers[1] = self.registers[0] >> self.combo_value(op),
            CDV(op) => self.registers[2] = self.registers[0] >> self.combo_value(op),
        }
        self.pc += 1;
    }
}

fn stars(mut program: Data) {
    let initial_registers = program.registers.clone();
    while program.pc < program.instructions.len() {
        program.execute_ins();
    }
    println!(
        "Star1: {:?}",
        program
            .output
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    // println!(
    //     "registers: {:?}, instructions: {:?}, output: {:?}, pc: {:?}, text_program: {:?}",
    //     program.registers, program.instructions, program.output, program.pc, program.text_program
    // );
    // let mut a = 1;
    // loop {
    //     program.registers = initial_registers;
    //     println!("a: {a}");
    //     program.registers[0] = a;
    //     program.pc = 0;
    //     program.output = Vec::new();
    //     while program.pc < program.instructions.len() {
    //         program.execute_ins();
    //     }
    //     println!("output: {:?}", program.output);
    //     println!("text_program: {:?}", program.text_program);
    //     if program.output == program.text_program {
    //         println!("Star2: {a}");
    //         break;
    //     }
    //     if program.output.len() < program.text_program.len() {
    //         a <<= 1;
    //     } else {
    //         a += 1;
    //     }
    // }
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
