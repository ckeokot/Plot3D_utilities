use ndarray::{Array3,Axis};

pub struct Block{
    pub x:Array3<f64>,
    pub y:Array3<f64>,
    pub z:Array3<f64>,
    pub imax:usize,
    pub jmax:usize,
    pub kmax:usize
}

impl Block {
    pub fn new (x:Array3<f64>,y:Array3<f64>,z:Array3<f64>) -> Self {
        let imax = x.len_of(Axis(0));
        let jmax = y.len_of(Axis(1));
        let kmax = y.len_of(Axis(2));
        Block {x,y,z,imax,jmax,kmax}
    }
}