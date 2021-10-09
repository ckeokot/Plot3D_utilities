use std::fs::File;
use std::io::{Read, Cursor,Error, BufReader, BufRead};
use ndarray::{Array3,ArrayBase};
use byteorder::{BigEndian, ReadBytesExt};

use super::block::Block;


fn read_chunk(block_data:&Vec<f64>, imax:usize,jmax:usize,kmax:usize) -> Array3<f64> {
    let mut data:Array3<f64> = ArrayBase::zeros((imax,jmax,jmax));
    let mut offset:usize = 0;

    for i in 0..imax{
        for j in 0..jmax{
            for k in 0..kmax{
                data[[i,j,k]] = block_data[offset];
                offset+=1
            }
        }
    }
    return data;
}

pub fn read_plot3d(filename:&str, binary:Option<bool>, big_endian:Option<bool>) -> Result<(), Error> {
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        println!("{}", line?);
    }
    Ok(())
}
