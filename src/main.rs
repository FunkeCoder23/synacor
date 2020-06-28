use std::fs::File;
use std::io::prelude::*;

pub mod instruction;
mod vm;

fn main() -> std::io::Result<()> {
    let mut file = File::open("challenge.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    //convert [lower, upper] u8 bytes to u16 {upper lower}
    let mut program = Vec::new();

    for i in (0..buffer.len() - 1).step_by(2).skip(2) {
        let word = (buffer[i] as u16 & 0x00FF) | (((buffer[i + 1] as u16) << 8) & 0xFF00);
        program.push(word);
    }
    let mut arch_vm = vm::VM::new();
    arch_vm.program(program);
    arch_vm.run();

    // println!("{:?}", &buffer[..]);
    Ok(())
}

#[test]
fn test_read_bytes() {
    let mut file = File::open("src/test2.bin").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    //convert [lower, upper] u8 bytes to u16 {upper lower}
    for i in (0..buffer.len() - 1).step_by(2) {
        //.skip(2) {
        let word = (buffer[i] as u16 & 0x00FF) | (((buffer[i + 1] as u16) << 8) & 0xFF00);
        print!("{}, ", word);
    }
}
