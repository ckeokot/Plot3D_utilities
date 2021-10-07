use std::fs::File;
use std::io::{Read, Error, BufReader, BufRead};
use super::block::Block;


pub fn read_plot3d(filename:&str, binary:Option<bool>, big_endian:Option<bool>) -> Result<(), Error> {
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        println!("{}", line?);
    }
    Ok(())
}
