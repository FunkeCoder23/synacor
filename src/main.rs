use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("challenge.bin")?;
    let rdr = BufReader::new(file);
    for line in rdr.lines(){
        println!("{:?} ",line?);
    }
    Ok(())
}