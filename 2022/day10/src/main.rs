mod util;

#[cfg(test)]
mod tests {
    use super::*;

    fn check_1a(filename: &str, cycle: usize, answer: i64) {
        let input = util::get_input_from_file(filename);
        let mut cpu = CPU::new();
        let prog = compile_program(&input);
        cpu.load_program(prog);

        let result = cpu.get_x_during_cycle(cycle);
        assert_eq!(result, answer);
    }

    fn check_1b(filename: &str, cycle: usize, answer: i64) {
        let input = util::get_input_from_file(filename);
        let mut cpu = CPU::new();
        let prog = compile_program(&input);
        cpu.load_program(prog);

        let result = cpu.get_signal_strength(cycle);
        assert_eq!(result, answer);
    }

    fn run_1(filename: &str, answer: i64) {
        let input = util::get_input_from_file(filename);
        let prog = compile_program(&input);

        let mut cpu = CPU::new();
        cpu.load_program(prog);

        let result = cpu.get_signal_strength_sum([20, 60, 100, 140, 180, 220]);
        assert_eq!(result, answer);
    }
    #[test]
    fn test_1a() {
        check_1a("example_a.txt", 1, 1);
        check_1a("example_a.txt", 2, 1);
        check_1a("example_a.txt", 3, 1);
        check_1a("example_a.txt", 4, 4);
        check_1a("example_a.txt", 5, 4);
        check_1a("example_a.txt", 6, -1);
    }
    
    #[test]
    fn test_1b() {
        check_1b("example_b.txt", 20, 420);
        check_1b("example_b.txt", 60, 1140);
        check_1b("example_b.txt", 100, 1800);
        check_1b("example_b.txt", 140, 2940);
        check_1b("example_b.txt", 180, 2880);
        check_1b("example_b.txt", 220, 3960);
    }

    #[test]
    fn test_1() {
        run_1("example_b.txt", 13140);
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    AddX(i64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Ready,
    Running(usize),
    Halt,
}

#[derive(Debug)]
struct CPU {
    x: i64,
    cc: usize,
    pc: usize,
    current_instruction: Option<Instruction>,
    state: State,
    program: Vec<Instruction>,
}

impl CPU {
    fn new() -> CPU {
        CPU { x: 1, cc: 0, pc: 0, state: State::Ready,
              current_instruction: None, program: Vec::new()}
    }

    fn tick(&mut self) {

        if self.pc >= self.program.len() {
            self.state = State::Halt;
        }

        self.cc += 1;

        match self.state {
            State::Ready => {
                self.current_instruction = Some(self.program[self.pc]);
                self.start_inst();
            }
            State::Running(_) => {
                self.continue_inst();
            }
            State::Halt => {
                return;
            }
        }
    }

    fn tock(&mut self) {

        match self.state {
            State::Ready => {
                panic!("Invalid Ready state at end of cycle.")
            },
            State::Running(duration) => {
                if duration == 0 {
                    panic!("Running duration 0 before end of cycle");
                }
                else if duration == 1 {
                    self.resolve_inst();
                    self.current_instruction = None;
                    self.pc += 1;
                    self.state = State::Ready;
                }
                else {
                    self.state = State::Running(duration-1);
                }
            }
            State::Halt => {
                return;
            }
        }
    }

    fn start_inst(&mut self)
    {
        match self.current_instruction.unwrap() {
            Instruction::Noop => {
                self.state = State::Running(1);
            },
            Instruction::AddX(_) => {
                self.state = State::Running(2);
            }
        }
    }
    
    fn continue_inst(&mut self)
    {
    }
    
    fn resolve_inst(&mut self)
    {
        match self.current_instruction.unwrap() {
            Instruction::AddX(val) => {
                self.x += val;
            }
            _ => {}
        }
    }

    fn load_program(&mut self, prog: Vec<Instruction>)
    {
        self.program = prog;
    }

    fn get_x_during_cycle(&mut self, cycle: usize) -> i64 {
        for _ in self.cc..(cycle-1) {
            self.tick();
            self.tock();
        }
        self.tick();
        let x = self.x;
        self.tock();

        x
    }
    
    fn get_signal_strength(&mut self, cycle: usize) -> i64 {
        let x = self.get_x_during_cycle(cycle);
        x * (self.cc as i64)
    }
    
    fn get_signal_strength_sum(&mut self, cycles: [usize; 6]) -> i64 {
        
        let mut sum = 0;

        for c in cycles {
            sum += self.get_signal_strength(c);
        }

        sum
    }

    fn reset(&mut self) {
        self.x = 1;
        self.cc = 0;
        self.pc = 0;
        self.current_instruction = None;
        self.state = State::Ready;
    }

    fn draw(&mut self) -> String {
        let mut crt = Vec::new();

        self.tick();

        let mut hpos = 0;

        while self.state != State::Halt {

            if (self.x - hpos).abs() <= 1 {
                crt.push('#');
            }
            else {
                crt.push('.');
            }

            if hpos < 39 {
                hpos += 1;
            }
            else {
                crt.push('\n');
                hpos = 0;
            }

            self.tock();
            self.tick();
        }

        crt.into_iter().collect()
    }
}

fn compile_program(input: &str) -> Vec<Instruction> {
    let mut prog = Vec::new();

    for line in input.lines() {
        let toks: Vec<&str> = line.split_whitespace().collect();

        if toks[0] == "noop" {
            prog.push(Instruction::Noop);
        }
        else if toks[0] == "addx" {
            let val: i64 = toks[1].parse().unwrap();
            prog.push(Instruction::AddX(val));
        }
    }

    prog
}

fn main() {
    let input = util::get_input(2022, 10);
    let prog = compile_program(&input);

    let mut cpu = CPU::new();
    cpu.load_program(prog);

    let s = cpu.get_signal_strength_sum([20, 60, 100, 140, 180, 220]);

    println!("Signal strength sum is: {}", s);

    cpu.reset();
    let output = cpu.draw();
    println!("{}", output);
}
