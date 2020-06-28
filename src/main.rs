use std::fs::File;
use std::io::prelude::*;

pub mod instruction;
mod vm;

fn main() -> std::io::Result<()> {
    let mut file = File::open("challenge.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut arch_vm = vm::VM::new();
    //convert [lower, upper] u8 bytes to u16 {upper lower}

    for i in (0..buffer.len() - 1).step_by(2) {
        let word = (buffer[i] as u16 & 0x00FF) | (((buffer[i + 1] as u16) << 8) & 0xFF00);
        arch_vm.program(word, i % 2);
    }
    // let mut fileout = File::create("challenge.txt")?;
    // arch_vm.dump(&mut fileout);
    arch_vm.run();

    // println!("{:?}", &buffer[..]);
    Ok(())
}

#[test]
fn test_read_bytes() {
    let mut file = File::open("challenge.bin").unwrap();
    let mut fileout = File::create("src/test_bytes.txt").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    // let mut outbuffer = Vec::new();
    let mut text = Vec::new();
    //convert [lower, upper] u8 bytes to u16 {upper lower}
    for i in (0..buffer.len() - 1).step_by(2) {
        //.skip(2) {
        let word = (buffer[i] as u16 & 0x00FF) | (((buffer[i + 1] as u16) << 8) & 0xFF00);
        text.clear();
        for ch in format!("{:b}", word).bytes() {
            text.push(ch);
        }
        fileout.write(&text.as_slice()).unwrap();
        fileout.write(",\n".as_bytes()).unwrap();
        // print!("{},\n", &W(text.clone()));
        // for outbyte in outbuffer {
        //     fileout.write(&outbuffer).unwrap();
        // }
    }
}

use std::fmt;
pub struct W(Vec<char>);

impl fmt::Display for W {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in self.0.clone() {
            write!(f, "{}", ch).unwrap();
        }
        Ok(())
    }
}

// impl W {
//     fn fmt::Display(&self){

//     }
// }
