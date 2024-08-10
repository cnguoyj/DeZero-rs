use ndarray::prelude::*;
use ndarray::{Array1};

#[derive(Debug)]
pub struct Variable<T> {
    pub data:Vec<T>,
}

impl <T> Variable<T> {
    fn new(value:Vec<T>) -> Self {
        Variable { data:value }
    }
}

trait Function<U> {
    fn call(&self,input:&Variable<U>) -> Variable<U> {
       self.forward(input)
    }

    fn forward(&self,x:&Variable<U>) -> Variable<U> {
        unimplemented!("forward not Implemented");
    }
}

struct Square {}

impl Function<f64> for Square {
    fn forward(&self,x:&Variable<f64>) -> Variable<f64> {
        let nd = Array1::from_vec(x.data.clone());
        let x_squared = nd.map(|nd| nd.powi(2));
        Variable { data:x_squared.into_raw_vec() }
    }
}

fn main() {
    let x = array![[10.0,20.0],[30.0,40.0]];
    println!("x {:?}",x);
    let y = Variable::new(x.into_raw_vec());
    println!("y {:?}",y);

    let square_fun = Square {};
    let var = square_fun.call(&y);

    println!("var {:?}",var);

    let arr = Array2::from_shape_vec((2, 2), var.data).unwrap();
    println!("ndarray: {:?}", arr);
}
