use ndarray::prelude::*;
use ndarray::{Array, Array1, Array2, Axis};

pub struct Variable<T> {
    pub data:T,
}

impl <T> Variable<T> {
    fn new(value:T) -> Self {
        Variable { data:value }
    }
}

fn main() {
    let x = array![[1.0]];
    println!("x {:?}",x);
}
