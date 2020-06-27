use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod instruction;
mod vm;

fn main() -> std::io::Result<()> {
    let file = File::open("src/test.bin")?;
    let rdr = BufReader::new(file);
    for line in rdr.lines() {
        for op in line.iter() {
            println!("{:?}\n", op);
        }
    }
    Ok(())
}
