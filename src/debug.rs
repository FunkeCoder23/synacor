use crate::instruction::Opcode;
use crate::vm;

use std::fs::File;
use std::io::Write;

impl vm::VM {
    pub fn dump(&self, output: &mut File) {
        let mut file = File::open("debug.txt").unwrap();
        loop {
            print!("{}: ", self.pc + 1);

            // If our program counter has exceeded the length of the program itself, something has
            // gone awry
            if self.pc >= self.memory.len() {
                break;
            }

            match self.decode_opcode() {
                Opcode::HALT => {
                    write!(file, "HLT encountered");
                }
                Opcode::SET => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    write!(file, format!("SET {} {}", a, b));
                }
                Opcode::PUSH => {
                    let a = self.next_bits();
                    write!(file, format!("SET {}", a));
                }
                Opcode::POP => {
                    let a = self.next_bits();
                    write!(file, format!("SET {}", a));
                }
                Opcode::EQ => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    write!(file, format!("SET {} {} {}", a, b, c));
                }
                Opcode::GT => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    write!(file, format!("SET {} {} {}", a, b, c));
                }
                Opcode::JMP => {
                    let a = self.next_bits();
                    write!(file, format!("SET {}", a));
                }
                Opcode::JNZ => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    write!(file, format!("SET {} {}", a, b));
                }
                Opcode::JZ => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    write!(file, format!("SET {} {}", a, b));
                }
                Opcode::ADD => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    write!(file, format!("SET {} {} {}", a, b, c));
                }
                Opcode::MULT => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    write!(file, format!("SET {} {} {}", a, b, c));
                }
                Opcode::MOD => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    write!(file, format!("SET {} {} {}", a, b, c));
                }
                Opcode::AND => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    write!(file, format!("SET {} {} {}", a, b, c));
                }
                Opcode::OR => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    write!(file, format!("SET {} {} {}", a, b, c));
                }
                Opcode::NOT => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    write!(file, format!("SET {} {}", a, b));
                }
                Opcode::RMEM => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    write!(file, format!("SET {} {}", a, b));
                }
                Opcode::WMEM => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    write!(file, format!("SET {} {}", a, b));
                }
                Opcode::CALL => {
                    let a = self.next_bits();
                    write!(file, format!("SET {}", a));
                }
                Opcode::RET => {
                    write!(file, format!("RET"));
                }
                Opcode::OUT => {
                    let a = self.next_bits();
                    write!(file, format!("SET {}", a));
                }
                Opcode::IN => {
                    let a = self.next_bits();
                    write!(file, format!("SET {}", a));
                }
                Opcode::NOOP => {
                    write!(file, "NOOP");
                }

                val => {
                    write!(file, "Unrecognized opcode ({:?}) found! Terminating!", val);
                    return;
                }
            }
        }
    }
}
