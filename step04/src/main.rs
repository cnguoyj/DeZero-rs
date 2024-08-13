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

struct Derivatives {}

impl Function<f64> for Derivatives {
    fn forward(&self,x:&Variable<f64>) -> Variable<f64> {
        let A = Square {};
        let B = Exp {};
        let C = Square {};

        let a = A.call(x);
        let b = B.call(&a);
        let c = C.call(&b);

        c
    }
}

fn numerical_diff(f:impl Function<f64>,x:&Variable<f64>,eps:f64) -> Variable<f64> {
    let var = Array1::from_vec(x.data.clone());
    let beh_var = &var - eps;
    let bef_var = &var + eps;
    let x0 = Variable::new(beh_var.into_raw_vec());
    let x1 = Variable::new(bef_var.into_raw_vec());

    let y0 = f.call(&x0);
    let y1 = f.call(&x1);

    let y0_nd = Array1::from_vec(y0.data.clone());
    let y1_nd = Array1::from_vec(y1.data.clone());

    let value = (y1_nd - y0_nd)/(2.0*eps);

    Variable { data:value.into_raw_vec() }
}

fn main() {
    let nd_x = array![[2.0]];
    println!("nd_x {:?}",nd_x);
    let x = Variable::new(nd_x.into_raw_vec());
    println!("x {:?}",x);

    let square_fun = Square {};
    let dy = numerical_diff(square_fun,&x,1e-4);
    println!("dy {:?}",dy);
    let nd_dy = Array1::from_vec(dy.data.clone());
    println!("nd_dy: {:?}", nd_dy);

    let nd_x = array![[0.5]];
    println!("nd_x {:?}",nd_x);
    let x = Variable::new(nd_x.into_raw_vec());
    println!("x {:?}",x);

    let derivatives_fun = Derivatives {};
    let dy = numerical_diff(derivatives_fun,&x,1e-4);
    println!("dy {:?}",dy);
    let nd_dy = Array1::from_vec(dy.data.clone());
    println!("nd_dy: {:?}", nd_dy);
}