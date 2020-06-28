use crate::instruction::Opcode;
use crate::vm;

use std::fs::File;
use std::io::Write;

impl vm::VM {
    #[allow(dead_code)]
    pub fn dump(&mut self, output: &mut File) {
        loop {
            write!(output, "{}: ", self.pc).unwrap();

            // If our program counter has exceeded the length of the program itself, something has
            // gone awry
            if self.pc >= self.memory.len() {
                break;
            }

            match self.decode_opcode() {
                Opcode::HALT => {
                    writeln!(output, "HLT").unwrap();
                }
                Opcode::SET => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    writeln!(output, "SET {} {}", a, b).unwrap();
                }
                Opcode::PUSH => {
                    let a = self.next_bits();
                    writeln!(output, "PUSH {}", a).unwrap();
                }
                Opcode::POP => {
                    let a = self.next_bits();
                    writeln!(output, "POP {}", a).unwrap();
                }
                Opcode::EQ => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    writeln!(output, "EQ {} {} {}", a, b, c).unwrap();
                }
                Opcode::GT => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    writeln!(output, "GT {} {} {}", a, b, c).unwrap();
                }
                Opcode::JMP => {
                    let a = self.next_bits();
                    writeln!(output, "JMP {}", a).unwrap();
                }
                Opcode::JNZ => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    writeln!(output, "JNZ {} {}", a, b).unwrap();
                }
                Opcode::JZ => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    writeln!(output, "JZ {} {}", a, b).unwrap();
                }
                Opcode::ADD => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    writeln!(output, "ADD {} {} {}", a, b, c).unwrap();
                }
                Opcode::MULT => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    writeln!(output, "MULT {} {} {}", a, b, c).unwrap();
                }
                Opcode::MOD => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    writeln!(output, "MOD {} {} {}", a, b, c).unwrap();
                }
                Opcode::AND => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    writeln!(output, "AND {} {} {}", a, b, c).unwrap();
                }
                Opcode::OR => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    let c = self.next_bits();
                    writeln!(output, "OR {} {} {}", a, b, c).unwrap();
                }
                Opcode::NOT => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    writeln!(output, "NOT {} {}", a, b).unwrap();
                }
                Opcode::RMEM => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    writeln!(output, "RMEM {} {}", a, b).unwrap();
                }
                Opcode::WMEM => {
                    let a = self.next_bits();
                    let b = self.next_bits();
                    writeln!(output, "WMEM {} {}", a, b).unwrap();
                }
                Opcode::CALL => {
                    let a = self.next_bits();
                    writeln!(output, "CALL {}", a).unwrap();
                }
                Opcode::RET => {
                    writeln!(output, "RET").unwrap();
                }
                Opcode::OUT => {
                    let a = self.next_bits();
                    writeln!(output, "OUT {}", a as u8 as char).unwrap();
                }
                Opcode::IN => {
                    let a = self.next_bits();
                    writeln!(output, "IN {}", a).unwrap();
                }
                Opcode::NOOP => {
                    writeln!(output, "NOOP").unwrap();
                }

                val => {
                    writeln!(output, "Unrecognized opcode ({:?}) found!", val).unwrap();
                }
            }
        }
    }
}
#[allow(unused_imports)]
use std::io::Read;

#[test]
fn test_dump() {
    let mut file = File::open("DONTTOUCH/challenge.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut arch_vm = vm::VM::new();
    //convert [lower, upper] u8 bytes to u16 {upper lower}
    for i in (0..buffer.len() - 1).step_by(2) {
        let word = (buffer[i] as u16 & 0x00FF) | (((buffer[i + 1] as u16) << 8) & 0xFF00);
        // println!("Programming {} into {}", word, i / 2);
        arch_vm.program(word, i / 2);
    }
    let mut fileout = File::create("debug.txt").unwrap();
    arch_vm.dump(&mut fileout);
    // println!("{:?}", &buffer[..]);
}

use permutohedron::heap_recursive;
#[test]
fn solve() {
    let coins: [u16; 5] = [2, 3, 5, 7, 9];
    let mut o = [0, 1, 2, 3, 4];

    let mut perms = Vec::new();
    let name = ["red", "corroded", "shiny", "concave", "blue"];

    heap_recursive(&mut o, |permutation| perms.push(permutation.to_vec()));
    // println!("{:?}", perms[1]);
    for perm in perms {
        let res = coins[perm[0]] + coins[perm[1]] * coins[perm[2]].pow(2) + coins[perm[3]].pow(3)
            - coins[perm[4]];
        if res == 399 {
            println!("{}", res);
            for num in perm.iter() {
                println!("{}", name[*num]);
            }
        }
    }

    // _ + _ * _^2 + _^3 - _ = 399
}
