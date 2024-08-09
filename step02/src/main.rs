use ndarray::prelude::*;
use ndarray::{Array,Array1};
use ndarray::OwnedRepr;

#[derive(Debug)]
pub struct Variable<T> {
    pub data:T,
}

impl <T> Variable<T> {
    fn new(value:T) -> Self {
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

impl Function<Array1<f64>> for Square {
    fn forward(&self,x:&Variable<Array1<f64>>) -> Variable<Array1<f64>> {
        let data = &x.data;
        let x_squared = data.map(|data| data.powi(2));
        Variable { data:x_squared }
    }
}

fn main() {
    let x: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> = array![[10.0]];
    let y = Variable::new(x);

    let square_fun = Square {};
    let var = square_fun.call(y);

    //println!("var {:?}",var);
}
