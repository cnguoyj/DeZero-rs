use ndarray::prelude::*;
use ndarray::{Array};

#[derive(Debug)]
pub struct Variable<T> {
    pub data:Vec<T>,
}

impl <T> Variable<T> {
    fn new(value:Vec<T>) -> Self {
        Variable { data:value }
    }
}

fn main() {
    let x = array![[1.0]];
    println!("x {:?}",x);
    let y = Variable::new(x.into_raw_vec());
    println!("y {:?}",y);
}
