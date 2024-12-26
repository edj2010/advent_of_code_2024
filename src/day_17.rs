use advent_of_code::parse::{parsers, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Combo {
    Literal(u64),
    A,
    B,
    C,
}

impl Combo {
    fn parse(operand: u64) -> Self {
        match operand {
            0..=3 => Self::Literal(operand),
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("invalid operand {}", operand),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv(Combo),
    Bxl(u64),
    Bst(Combo),
    Jnz(u64),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl Instruction {
    fn parse(opcode: u64, operand: u64) -> Self {
        let combo = Combo::parse(operand);
        match opcode {
            0 => Instruction::Adv(combo),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(combo),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc,
            5 => Instruction::Out(combo),
            6 => Instruction::Bdv(combo),
            7 => Instruction::Cdv(combo),
            _ => panic!("invalid opcode {}", opcode),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn read_combo(self, c: Combo) -> u64 {
        match c {
            Combo::Literal(r) => r,
            Combo::A => self.a,
            Combo::B => self.b,
            Combo::C => self.c,
        }
    }
}

#[derive(Clone)]
struct Computer {
    registers: Registers,
    instruction_pointer: usize,
    program: Vec<u64>,
}

impl Computer {
    fn new(registers: Registers, program: Vec<u64>) -> Self {
        Computer {
            registers,
            instruction_pointer: 0,
            program,
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Option<u64> {
        match instruction {
            Instruction::Adv(operand) => {
                self.registers.a >>= self.registers.read_combo(operand);
                None
            }
            Instruction::Bxl(operand) => {
                self.registers.b ^= operand;
                None
            }
            Instruction::Bst(operand) => {
                self.registers.b = self.registers.read_combo(operand) % 8;
                None
            }
            Instruction::Jnz(operand) => {
                if self.registers.a != 0 {
                    self.instruction_pointer = operand as usize;
                };
                None
            }
            Instruction::Bxc => {
                self.registers.b ^= self.registers.c;
                None
            }
            Instruction::Out(operand) => Some(self.registers.read_combo(operand) % 8),
            Instruction::Bdv(operand) => {
                self.registers.b = self.registers.a >> self.registers.read_combo(operand);
                None
            }
            Instruction::Cdv(operand) => {
                self.registers.c = self.registers.a >> self.registers.read_combo(operand);
                None
            }
        }
    }

    fn step(&mut self) -> Option<u64> {
        if let Some(&opcode) = self.program.get(self.instruction_pointer)
            && let Some(&operand) = self.program.get(self.instruction_pointer + 1)
        {
            self.instruction_pointer += 2;
            // println!(
            //     "{:?}\n {} {:?}",
            //     self.registers,
            //     self.instruction_pointer,
            //     Instruction::parse(opcode, operand)
            // );
            self.execute(Instruction::parse(opcode, operand))
        } else {
            None
        }
    }

    fn is_done(&self) -> bool {
        self.instruction_pointer >= self.program.len()
    }

    fn run(&mut self) -> Vec<u64> {
        let mut outputs = Vec::new();
        while !self.is_done() {
            if let Some(out) = self.step() {
                outputs.push(out);
            }
        }
        outputs
    }

    fn initialize(&mut self, a: u64, b: u64, c: u64, instruction_pointer: usize) {
        self.registers.a = a;
        self.registers.b = b;
        self.registers.c = c;
        self.instruction_pointer = instruction_pointer;
    }

    fn run_in_reverse(
        &mut self,
        final_register_a: u64,
        desired_output: &[u64],
        desired_output_from_idx: usize,
    ) -> Option<u64> {
        // We'll make the simplifying assumption that the program is a simple loop that terminates when register A = 0
        // We'll also make the simplifying assumption that B and C are both initialized based on A in the core program loop before use
        // So in particular previous values of B and C do not matter, and the only information carried through is via A
        // Also note, the only instruction that modifies A is adv, which in practice we only get with literal combo args, so we can search for instances of it first to bound our search space for A
        if desired_output.len() == 0 {
            return Some(final_register_a);
        }
        let adv_shift = self
            .program
            .iter()
            .enumerate()
            .step_by(2)
            .find(|(_, &i)| i == 0)
            .map(|(idx, _)| self.program[idx + 1])
            .unwrap();
        let min_a = final_register_a << adv_shift;
        let max_a = (final_register_a + 1) << adv_shift;

        (min_a..max_a)
            .filter_map(|a| {
                self.initialize(a, 0, 0, 0);
                let outputs = self.run(); // probably need to only check the first output, but checking everything helped catch quiet shl overflow issues so this feels more robust
                if outputs == desired_output[desired_output_from_idx..] {
                    if desired_output_from_idx == 0 {
                        Some(a)
                    } else {
                        self.run_in_reverse(a, &desired_output, desired_output_from_idx - 1)
                    }
                } else {
                    None
                }
            })
            .min()
    }
}

fn parse(input: &str) -> Computer {
    parsers::tag("Register A: ")
        .ignore_and_then(parsers::number())
        .pair("\nRegister B: ", parsers::number())
        .pair("\nRegister C: ", parsers::number())
        .map(|((a, b), c)| Registers { a, b, c })
        .pair(
            "\n\nProgram: ",
            parsers::number().list(",").map(|v| v.collect::<Vec<u64>>()),
        )
        .map(|(registers, instructions)| Computer::new(registers, instructions))
        .skip_tag("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> String {
    parse(input)
        .run()
        .into_iter()
        .map(|n| format!("{}", n))
        .fold("".to_owned(), |acc, n| {
            if acc == "" {
                n
            } else {
                format!("{acc},{n}")
            }
        })
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    let mut computer = parse(input);
    let target = computer.program.clone();
    computer
        .run_in_reverse(0, &target, target.len() - 1)
        .unwrap()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    const DAY: Day = Day::Day17;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
            ),
            117440
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            "6,0,6,3,0,2,3,1,6"
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            236539226447469
        );
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| {
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| {
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }
}
