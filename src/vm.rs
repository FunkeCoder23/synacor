const DEBUG: bool = true;

/// == Architecture ==
/// - three storage regions
/// - memory with 15-bit address space storing 16-bit values
/// - eight registers
/// - an unbounded stack which holds individual 16-bit values
/// - all numbers are unsigned integers 0..32767 (15-bit)
/// - all math is modulo 32768; 32758 + 15 => 5
use crate::instruction::*;

#[allow(dead_code)]
pub struct VM {
    registers: [u16; 8],
    stack: Vec<u16>,
    memory: [u16; 32768],
    pc: usize,
    program: Vec<u16>,
}

#[allow(dead_code)]
impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 8],
            stack: Vec::new(),
            memory: [0; 32768],
            program: Vec::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            // If our program counter has exceeded the length of the program itself, something has
            // gone awry
            if self.pc >= self.program.len() {
                break;
            }
            match self.decode_opcode() {
                Opcode::HALT => {
                    println!("HLT encountered");
                    return;
                }
                Opcode::NOOP => {
                    continue;
                }
                Opcode::OUT => {
                    let ch = self.next_bits() as u8 as char;
                    if DEBUG {
                        println!("OUT called\nPrinting {:?} to terminal", ch);
                    }
                    print!("{}", ch);
                }
                Opcode::ADD => {
                    let dest = self.next_bits() % 32768;
                    let (is_lit_a, a) = VM::check_num(self.next_bits());
                    let (is_lit_b, b) = VM::check_num(self.next_bits());

                    let val_a = if is_lit_a { a } else { self.memory[a as usize] };
                    let val_b = if is_lit_b { b } else { self.memory[b as usize] };
                    if DEBUG {
                        println!(
                            "ADD called\n Adding {}({}) and {}({}) and storing in {}",
                            val_a, a, val_b, b, dest
                        )
                    }
                    self.memory[dest as usize] = val_a + val_b;
                }
                _ => {
                    println!("Unrecognized opcode found! Terminating!");
                    return;
                }
            }
        }
    }

    /// format_num: returns if num is literal and the value
    ///
    /// - each number is stored as a 16-bit little-endian pair (low byte, high byte)
    /// - numbers 0..32767 mean a literal value
    /// - numbers 32768..32775 instead mean registers 0..7
    /// - numbers 32776..65535 are invalid
    fn check_num(num: u16) -> (bool, u16) {
        let mut is_lit = false;
        if num < 32767 {
            is_lit = true;
        } else if num > 32776 {
            panic!("Invalid number");
        }
        (is_lit, num % 32768)
    }
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_bits(&mut self) -> u16 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_memory_length() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers.len(), 8);
        assert_eq!(test_vm.memory.len(), 32768)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_set() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    /// - The program "9,32768,32769,4,19,32768" occupies six memory addresses and should:
    /// - Store into register 0 the sum of 4 and the value contained in register 1.
    /// - Output to the terminal the character with the ascii code contained in register 0.
    #[test]
    fn test_sample_prog() {
        let mut test_vm = VM::new();
        let test_bytes = vec![9, 32768, 32769, 4, 19, 32768];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.memory[0], 4);
    }
}
