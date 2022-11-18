
pub struct Intcode {
    instructions: Vec<i64>,
    output: Option<i64>,
    input: Vec<i64>,
    state: State

}

#[derive(PartialEq, Debug)]
pub enum State {
    Ready,
    Processing,
    InputRequired,
    Halted,
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Add(char, char, char),
    Multiply(char, char, char),
    Input(char),
    Output(char),
    JumpIfTrue(char, char),
    JumpIfFalse(char, char),
    LessThan(char, char, char),
    Equals(char, char, char),
    Halt
}

/*
        Instructions
        1 1 0 0 2
        A B C D E
        0 1 2 3 4

        A: 3rd param
        B: 2nd param
        C: 1st param
        D,E: optcode
        1: immediate mode
        0: position mode

   */
fn process_instruction(value: i64) -> Instruction {

    let mut s_chars: Vec<char> = value.to_string().chars().collect();

    while s_chars.len() < 5 {
        s_chars.insert(0, '0');
    }

    let mut instr = String::new();

    instr.push(s_chars[3]);
    instr.push(s_chars[4]);

    match instr.as_str() {
        "01" => return Instruction::Add(s_chars[2], s_chars[1], s_chars[0]),
        "02" => return Instruction::Multiply(s_chars[2], s_chars[1], s_chars[0]),
        "03" => return Instruction::Input(s_chars[2]),
        "04" => return Instruction::Output(s_chars[2]),
        "05" => return Instruction::JumpIfTrue(s_chars[2], s_chars[1]),
        "06" => return Instruction::JumpIfFalse(s_chars[2], s_chars[1]),
        "07" => return Instruction::LessThan(s_chars[2], s_chars[1], s_chars[0]),
        "08" => return Instruction::Equals(s_chars[2], s_chars[1], s_chars[0]),
        "99" => return Instruction::Halt,
        _ => panic!("Invalid optcode value."),
    }
}

fn get_index(instructions: &Vec<i64>, pos: usize, mode: char) -> usize {
    if mode == '1' {
        return pos;
    }

    instructions[pos] as usize
}

impl Intcode {
    pub fn new(instructions: Vec<i64>) -> Intcode {
        let input = Vec::new();
        Intcode {instructions, output: None, input, state: State::Ready}
    }

    pub fn get_output(&self) -> Option<i64> {
        self.output
    }

    pub fn add_input(&mut self, value: i64) {
        self.input.push(value);
    }

    pub fn new_instructions(&mut self, instructions: Vec<i64>) {
        self.instructions = instructions;
    }

    pub fn get_value_at(&self, index: usize) -> Option<i64> {
        if index >= self.instructions.len() {
            return None;
        }

        Some(self.instructions[index])
    }

    pub fn get_state(&self) -> &str {
        match self.state {
            State::Ready => return "ready",
            State::Processing => return "processing",
            State::InputRequired => "inputRequired",
            State::Halted => return  "halted",
        }
    }

    pub fn process(&mut self) {
        let mut pos: usize = 0;

        loop {
            self.state = State::Processing;
            let instr = process_instruction(self.instructions[pos]);
            match instr {
                Instruction::Add(p1,p2,p3) => {
                    let pos1 = get_index(&self.instructions, pos+1, p1);
                    let pos2 = get_index(&self.instructions, pos+2, p2);
                    let pos3 = get_index(&self.instructions, pos+3, p3);

                    self.instructions[pos3] = self.instructions[pos1] + self.instructions[pos2];
                    pos += 4;
                },
                Instruction::Multiply(p1,p2,p3) => {
                    let pos1 = get_index(&self.instructions, pos+1, p1);
                    let pos2 = get_index(&self.instructions, pos+2, p2);
                    let pos3 = get_index(&self.instructions, pos+3, p3);

                    self.instructions[pos3] = self.instructions[pos1] * self.instructions[pos2];
                    pos += 4;

                },
                Instruction::Input(p1) => {
                    if let Some(value) = self.get_input() {
                        let pos1 = get_index(&self.instructions, pos+1, p1);
                        self.instructions[pos1] = value;
                    } else {
                        panic!("No input!")
                    }
                    pos += 2;

                },
                Instruction::Output(p1) => {
                    let pos1 = get_index(&self.instructions, pos+1, p1);
                    self.output = Some(self.instructions[pos1]);
                    pos += 2;

                },
                Instruction::JumpIfTrue(p1,p2) => {
                    let idx1 = get_index(&self.instructions, pos+1, p1);
                    let idx2 = get_index(&self.instructions, pos+2, p2);

                    if self.instructions[idx1] > 0 {
                        pos = self.instructions[idx2] as usize;
                    } else {
                        pos += 3;
                    }
                },
                Instruction::JumpIfFalse(p1,p2) => {
                    let idx1 = get_index(&self.instructions, pos+1, p1);
                    let idx2 = get_index(&self.instructions, pos+2, p2);

                    if self.instructions[idx1] == 0 {
                        pos = self.instructions[idx2] as usize;
                    } else {
                        pos += 3;
                    }
                },
                Instruction::LessThan(p1,p2,p3) => {
                    let idx1 = get_index(&self.instructions, pos+1, p1);
                    let idx2 = get_index(&self.instructions, pos+2, p2);
                    let idx3 = get_index(&self.instructions, pos+3, p3);

                    if self.instructions[idx1] < self.instructions[idx2] {
                        self.instructions[idx3] = 1;
                    } else {
                        self.instructions[idx3] = 0;
                    }
                    pos += 4;
                },
                Instruction::Equals(p1,p2,p3) => {
                    let idx1 = get_index(&self.instructions, pos+1, p1);
                    let idx2 = get_index(&self.instructions, pos+2, p2);
                    let idx3 = get_index(&self.instructions, pos+3, p3);

                    if self.instructions[idx1] == self.instructions[idx2] {
                        self.instructions[idx3] = 1;
                    } else {
                        self.instructions[idx3] = 0;
                    }
                    pos += 4;
                },
                Instruction::Halt => {
                    self.state = State::Halted;
                    break
                },
            }

            if pos >= self.instructions.len() {
                panic!("Invalid intcode size!")
            }
        }
    }

    fn get_input(&mut self) -> Option<i64> {
        self.input.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_multiply_optcodes() {
        let optcodes = vec![1,1,1,4,99,5,6,0,99];

        let mut intcode = Intcode::new(optcodes);
        intcode.process();

        assert_eq!(intcode.get_value_at(0), Some(30));
    }

    #[test]
    fn load_new_optcodes() {
        let optcodes = vec![1,1,1,4,99,5,6,0,99];

        let mut intcode = Intcode::new(vec![99,0,0,0]);

        intcode.new_instructions(optcodes);
        intcode.process();

        assert_eq!(intcode.get_value_at(0), Some(30));
    }

    #[test]
    fn process_instruction_add() {
        let value = 1;
        let ans = process_instruction(value);

        assert!(matches!(ans, Instruction::Add('0', '0', '0')));
    }

    #[test]
    fn process_instruction_add_2() {
        let value = 10001;
        let ans = process_instruction(value);

        assert!(matches!(ans, Instruction::Add('0', '0', '1')));
    }

    #[test]
    fn process_instruction_multiply() {
        let value = 2;
        let ans = process_instruction(value);

        assert!(matches!(ans, Instruction::Multiply('0', '0', '0')));
    }

    #[test]
    fn process_instruction_multiply_2() {
        let value = 102;
        let ans = process_instruction(value);
        assert!(matches!(ans, Instruction::Multiply('1', '0', '0')));
    }

    #[test]
    fn process_instruction_multiply_3() {
        let value = 11002;
        let ans = process_instruction(value);

        assert!(matches!(ans, Instruction::Multiply('0', '1', '1')));
    }

    #[test]
    fn process_instruction_input_mode_0() {
        let value = 3;
        let ans = process_instruction(value);
        assert!(matches!(ans, Instruction::Input('0')));
    }

    #[test]
    fn process_instruction_input_mode_1() {
        let value = 103;
        let ans = process_instruction(value);

        assert!(matches!(ans, Instruction::Input('1')));
    }

    #[test]
    fn process_instruction_output_mode_0() {
        let value = 4;
        let ans = process_instruction(value);
        assert!(matches!(ans, Instruction::Output('0')));
    }

    #[test]
    fn process_instruction_output_mode_1() {
        let value = 104;
        let ans = process_instruction(value);

        assert!(matches!(ans, Instruction::Output('1')));
    }

    #[test]
    fn process_instruction_halt() {
        let value = 99;
        let ans = process_instruction(value);

        assert!(matches!(ans, Instruction::Halt));
    }

    #[test]
    fn jump_test_1() {
        let instr = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

        let mut intcode = Intcode::new(instr);
        intcode.add_input(1);
        intcode.process();


        assert_eq!(intcode.get_output(), Some(1));
    }

    #[test]
    fn jump_test_0() {
        let instr = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

        let mut intcode = Intcode::new(instr);
        intcode.add_input(0);
        intcode.process();


        assert_eq!(intcode.get_output(), Some(0));
    }

    #[test]
    fn e2e_test_1() {
        let instr = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut intcode = Intcode::new(instr);
        intcode.add_input(2);
        intcode.process();


        assert_eq!(intcode.get_output(), Some(999));
    }

    #[test]
    fn e2e_test_2() {
        let instr = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut intcode = Intcode::new(instr);
        intcode.add_input(8);
        intcode.process();


        assert_eq!(intcode.get_output(), Some(1000));
    }

    #[test]
    fn e2e_test_3() {
        let instr = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut intcode = Intcode::new(instr);
        intcode.add_input(34);
        intcode.process();

        assert_eq!(intcode.get_output(), Some(1001));
    }
}


