const DEBUG: bool = true;

/// == Architecture ==
/// - three storage regions
/// - memory with 15-bit address space storing 16-bit values
/// - eight registers
/// - an unbounded stack which holds individual 16-bit values
/// - all numbers are unsigned integers 0..32767 (15-bit)
/// - all math is modulo 32768; 32758 + 15 => 5
use crate::instruction::*;
use std::io;

#[allow(dead_code)]
pub struct VM {
    pub registers: [u16; 8],
    pub stack: Vec<u16>,
    pub memory: [u16; 32768],
    pub pc: usize,
}

#[allow(dead_code)]
impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 8],
            stack: Vec::new(),
            memory: [0; 32768],
            pc: 0,
        }
    }

    pub fn program(&mut self, instruction: u16, i: usize) {
        self.memory[i] = instruction;
    }

    pub fn run(&mut self) {
        let mut input = String::new();
        loop {
            if DEBUG {
                print!("{}: ", self.pc + 1);
            }
            // If our program counter has exceeded the length of the program itself, something has
            // gone awry
            if self.pc >= self.memory.len() {
                break;
            }
            match self.decode_opcode() {
                Opcode::HALT => {
                    println!("HLT encountered");
                    return;
                }
                Opcode::SET => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    if DEBUG {
                        println!(
                            "SET called\n\tstoring {}({}) in register {}",
                            val_b, b, reg_a
                        );
                    }
                    self.registers[reg_a as usize] = val_b;
                }
                Opcode::PUSH => {
                    let (is_lit_a, a) = VM::check_num(self.next_bits());
                    let val_a = if is_lit_a {
                        a
                    } else {
                        self.registers[a as usize]
                    };
                    if DEBUG {
                        println!("PUSH called\n\tStoring {}({}) in stack", val_a, a);
                    }
                    self.stack.push(val_a);
                }
                Opcode::POP => {
                    match self.stack.pop() {
                        None => {
                            if DEBUG {
                                println!("POP called\n\tEmpty stack, exiting");
                            }
                            return;
                        }
                        Some(val) => {
                            let reg_a = self.next_bits() % 32768;
                            if DEBUG {
                                println!("POP called\n\tStoring {} in {}", val, reg_a);
                            }
                            self.registers[reg_a as usize] = val;
                        }
                    };
                }
                Opcode::EQ => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let (is_lit_c, c) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    let val_c = if is_lit_c {
                        c
                    } else {
                        self.registers[b as usize]
                    };

                    if DEBUG {
                        println!(
                            "EQ called\n\tTesting {}({}) == {}({}) and storing in {}",
                            val_b, b, val_c, c, reg_a
                        )
                    }

                    self.registers[reg_a as usize] = (val_b == val_c) as u16;
                }
                Opcode::GT => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let (is_lit_c, c) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    let val_c = if is_lit_c {
                        c
                    } else {
                        self.registers[b as usize]
                    };
                    if DEBUG {
                        println!(
                            "GT called\n\tTesting {}({}) > {}({}) and storing in {}",
                            val_b, b, val_c, c, reg_a
                        )
                    }
                    self.registers[reg_a as usize] = (val_b > val_c) as u16;
                }
                Opcode::JMP => {
                    let (is_lit_a, a) = VM::check_num(self.next_bits());
                    let val_a = if is_lit_a {
                        a
                    } else {
                        self.registers[a as usize]
                    };
                    if DEBUG {
                        println!("JMP called\n\tPC set to {}({})", val_a, a)
                    }
                    self.pc = val_a as usize;
                }
                Opcode::JNZ => {
                    let (is_lit_a, a) = VM::check_num(self.next_bits());
                    let (is_lit_b, b) = VM::check_num(self.next_bits());

                    let val_a = if is_lit_a {
                        a
                    } else {
                        self.registers[a as usize]
                    };
                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };

                    if val_a != 0 {
                        if DEBUG {
                            println!(
                                "JNZ called\n\t{}({}) is nonzero, PC set to {}({})",
                                val_a, a, val_b, b
                            );
                        }
                        self.pc = val_b as usize;
                    } else {
                        if DEBUG {
                            println!("JNZ called\n\t{}({}) is zero, PC unchanged", val_a, a);
                        }
                    }
                }
                Opcode::JZ => {
                    let (is_lit_a, a) = VM::check_num(self.next_bits());
                    let val_a = if is_lit_a {
                        a
                    } else {
                        self.registers[a as usize]
                    };
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    if val_a == 0 {
                        if DEBUG {
                            println!(
                                "JZ called\n\t{}({}) is zero, PC set to {}({})",
                                val_a, a, val_b, b
                            );
                        }
                        self.pc = val_b as usize;
                    } else {
                        if DEBUG {
                            println!("JZ called\n\t{}({}) is nonzero, PC unchanged", val_a, a);
                        }
                    }
                }
                Opcode::ADD => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let (is_lit_c, c) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    let val_c = if is_lit_c {
                        c
                    } else {
                        self.registers[c as usize]
                    };
                    if DEBUG {
                        println!(
                            "ADD called\n\tAdding {}({}) and {}({}) and storing in {}",
                            val_b, b, val_c, c, reg_a
                        )
                    };
                    self.registers[reg_a as usize] = (val_b + val_c) % 32768;
                }
                Opcode::MULT => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let (is_lit_c, c) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    let val_c = if is_lit_c {
                        c
                    } else {
                        self.registers[c as usize]
                    };

                    if DEBUG {
                        println!(
                          "MULT called\n\tMultiplying {}({}) and {}({}) (%32768) and storing in {}",
                          val_b, b, val_c, c, reg_a
                      );
                    }
                    self.registers[reg_a as usize] = (val_b * val_c) % 32768;
                }
                Opcode::MOD => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let (is_lit_c, c) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    let val_c = if is_lit_c {
                        c
                    } else {
                        self.registers[c as usize]
                    };

                    if DEBUG {
                        println!(
                            "MOD called\n\tRemainder of {}({}) / {}({}) and storing in {}",
                            val_b, b, val_c, c, reg_a
                        );
                    }

                    self.registers[reg_a as usize] = val_b % val_c;
                }
                Opcode::AND => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let (is_lit_c, c) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    let val_c = if is_lit_c {
                        c
                    } else {
                        self.registers[c as usize]
                    };

                    if DEBUG {
                        println!(
                            "AND called\n\tBitwise AND of {}({}) and {}({}) and storing in {}",
                            val_b, b, val_c, c, reg_a
                        );
                    }

                    self.registers[reg_a as usize] = val_b & val_c;
                }
                Opcode::OR => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());
                    let (is_lit_c, c) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    let val_c = if is_lit_c {
                        c
                    } else {
                        self.registers[c as usize]
                    };
                    if DEBUG {
                        println!(
                            "OR called\n\tBitwise OR of {}({}) and {}({}) and storing in {}",
                            val_b, b, val_c, c, reg_a
                        );
                    }
                    self.registers[reg_a as usize] = val_b | val_c;
                }
                Opcode::NOT => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };
                    if DEBUG {
                        println!(
                            "OR called\n\tComplement of {}({}) and storing in {}",
                            val_b, b, reg_a
                        );
                    }
                    self.registers[reg_a as usize] = !val_b & 0x7FFF;
                }
                Opcode::RMEM => {
                    let reg_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b { b } else { self.memory[b as usize] };

                    if DEBUG {
                        println!(
                            "RMEM called\n\tReading mem {}({}) and storing in {}",
                            val_b, b, reg_a
                        );
                    }
                    self.registers[reg_a as usize] = val_b;
                }
                Opcode::WMEM => {
                    let mem_a = self.next_bits() % 32768;
                    let (is_lit_b, b) = VM::check_num(self.next_bits());

                    let val_b = if is_lit_b {
                        b
                    } else {
                        self.registers[b as usize]
                    };

                    if DEBUG {
                        println!("WMEM called\n\tWriting {}({}) to MEM[{}]", val_b, b, mem_a);
                    }
                    self.memory[mem_a as usize] = val_b;
                }
                Opcode::CALL => {
                    let (is_lit_a, a) = VM::check_num(self.next_bits());

                    let val_a = if is_lit_a {
                        a
                    } else {
                        self.registers[a as usize]
                    };

                    if DEBUG {
                        println!(
                            "CALL called\n\tWriting {} to stack and jumping to {}({})",
                            self.pc, val_a, a
                        );
                    }
                    self.stack.push(self.pc as u16);
                    self.pc = val_a as usize;
                }
                Opcode::RET => {
                    self.pc = match self.stack.pop() {
                        Some(val) => {
                            if DEBUG {
                                println!("RET called\n\tPopping {} from stack and jumping", val);
                            }
                            val as usize
                        }
                        None => {
                            if DEBUG {
                                println!("RET called\n\tEmpty stack, returning");
                            }
                            return;
                        }
                    };
                }
                Opcode::OUT => {
                    let (is_lit_a, a) = VM::check_num(self.next_bits());

                    let val_a = if is_lit_a {
                        a
                    } else {
                        self.registers[a as usize]
                    };
                    if DEBUG {
                        print!(
                            "OUT called\n\tPrinting {:?} to terminal: ",
                            val_a as u8 as char
                        );
                    }
                    print!("{}", val_a as u8 as char);
                    if DEBUG {
                        println!();
                    }
                }
                Opcode::IN => {
                    //TODO
                    match io::stdin().read_line(&mut input) {
                        Ok(n) => {
                            println!("{} bytes read", n);
                            println!("{}", input);
                        }
                        Err(error) => println!("error: {}", error),
                    }
                }
                Opcode::NOOP => {
                    if DEBUG {
                        println!("NOOP");
                    }
                    continue;
                }

                val => {
                    println!("Unrecognized opcode ({:?}) found! Terminating!", val);
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
    pub fn check_num(num: u16) -> (bool, u16) {
        let mut is_lit = false;
        if num < 32767 {
            is_lit = true;
        } else if num > 32776 {
            panic!("Invalid number");
        }
        (is_lit, num % 32768)
    }
    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.memory[self.pc]);
        self.pc += 1;
        return opcode;
    }

    pub fn next_bits(&mut self) -> u16 {
        let result = self.memory[self.pc];
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
        let test_bytes: Vec<u16> = vec![0, 0, 0, 0];
        for (i, bytes) in test_bytes.iter().enumerate() {
            test_vm.program(*bytes, i);
        }
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_set() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        for (i, bytes) in test_bytes.iter().enumerate() {
            test_vm.program(*bytes, i);
        }
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
        test_vm.registers[1] = 99;
        for (i, bytes) in test_bytes.iter().enumerate() {
            test_vm.program(*bytes, i);
        }
        test_vm.run();
        println!("Register 0 is : {}", test_vm.registers[0]);
        assert_eq!(test_vm.registers[0], 103);
    }
}
