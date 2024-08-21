use ndarray::prelude::*;
use ndarray::{Array1};

#[derive(Debug,Clone)]
pub struct Variable<T> {
    pub data:Vec<T>,
    pub grad:Vec<T>,
}

impl <T> Variable<T> {
    fn new(value:Vec<T>) -> Self {
        Variable { 
            data:value,
            grad:Vec::with_capacity(0),
        }
    }
}

trait Function<U> {
    fn call(&mut self,input:&Variable<U>) -> Variable<U> {
        let output = self.forward(input);
        self.update_input(input);
        output
    }

    fn forward(&self,x:&Variable<U>) -> Variable<U> {
        unimplemented!("forward not Implemented");
    }

    fn backward(&self,gy:&Variable<U>) -> Vec<U> {
        unimplemented!("forward not Implemented");
    }

     fn update_input(&mut self, input: &Variable<U>) {
        unimplemented!("forward not Implemented");
    }
}

struct Square {
    input:Variable<f64>,
}

impl Function<f64> for Square {
    fn forward(&self,x:&Variable<f64>) -> Variable<f64> {
        let nd_x = Array1::from_vec(x.data.clone());
        let x_squared = nd_x.map(|nd| nd.powi(2));
        Variable { 
            data:x_squared.into_raw_vec(),
            grad:Vec::with_capacity(0),
         }
    }

    fn backward(&self,gy:&Variable<f64>) -> Vec<f64> {
        let nd_input = Array1::from_vec(self.input.data.clone());
        let nd_gy = Array1::from_vec(gy.grad.clone());

        let nd_gx = 2.0*&nd_input*&nd_gy;

        nd_gx.into_raw_vec()
    }

    fn update_input(&mut self, input: &Variable<f64>) {
        self.input = input.clone();
    }
}

struct Exp {
    input:Variable<f64>,
}

impl Function<f64> for Exp {
    fn forward(&self,x:&Variable<f64>) -> Variable<f64> {
        let nd_x = Array1::from_vec(x.data.clone());
        let nd_exp = nd_x.map(|nd| nd.exp());
        Variable { 
            data:nd_exp.into_raw_vec(),
            grad:Vec::with_capacity(0),
        }
    }

    fn backward(&self,gy:&Variable<f64>) -> Vec<f64> {
        let nd_input = Array1::from_vec(self.input.data.clone());
        let nd_gy = Array1::from_vec(gy.grad.clone());

        let nd_exp = nd_input.map(|nd| nd.exp());
        let nd_gx = &nd_exp*&nd_gy;

        nd_gx.into_raw_vec()
    }

    fn update_input(&mut self, input: &Variable<f64>) {
        self.input = input.clone();
    }
}

fn main() {
    let mut A = Square {
        input:Variable::new(Vec::<f64>::with_capacity(0)), 
    };

    let mut B = Exp {
        input:Variable::new(Vec::<f64>::with_capacity(0)), 
    };

    let mut C = Square {
        input:Variable::new(Vec::<f64>::with_capacity(0)), 
    };

    let nd_x = array![[0.5]];
    println!("nd_x {:?}",nd_x);
    let mut x = Variable::new(nd_x.into_raw_vec());
    println!("x {:?}",x);

    let mut a = A.call(&x);
    let mut b = B.call(&a);
    let mut y = C.call(&b);
    println!("y {:?}",y.data);

    let nd_y = Array1::from_vec(y.data.clone());
    println!("nd_y: {:?}", nd_y);

    let nd_y_grad = array![[1.0]];
    y.grad = nd_y_grad.into_raw_vec();
    b.grad = C.backward(&y);
    a.grad = B.backward(&b);
    x.grad = A.backward(&a);
    println!("x {:?}",x);
}