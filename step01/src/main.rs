use ndarray::prelude::*;
use ndarray::{Array};

#[derive(Debug)]
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
    let y = Variable::new(x);
    println!("y {:?}",y);
}
