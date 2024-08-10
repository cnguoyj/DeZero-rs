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

struct Exp {}

impl Function<f64> for Exp {
    fn forward(&self,x:&Variable<f64>) -> Variable<f64> {
        let nd = Array1::from_vec(x.data.clone());
        let exp = nd.map(|nd| nd.exp());
        Variable { data:exp.into_raw_vec() }
    }
}


fn main() {
    let ndx = array![[0.5]];
    println!("ndx {:?}",ndx);
    let x = Variable::new(ndx.into_raw_vec());
    println!("x {:?}",x);

    let square_fun = Square {};
    let a = square_fun.call(&x);
    println!("a {:?}",a);

    let exp_fun = Exp {};
    let  b = exp_fun.call(&a);

    let y = square_fun.call(&b);

    let arr = Array1::from_vec(y.data.clone());
    println!("ndarray: {:?}", arr);
}