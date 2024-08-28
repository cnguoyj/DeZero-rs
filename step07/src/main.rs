use ndarray::prelude::*;
use ndarray::{Array1};

use std::rc::{Rc,Weak};
use std::cell::RefCell;

#[warn(unused_variables)]

#[derive(Debug,Clone)]
pub enum GradientFunction {
    Square(RefCell<Weak<Square>>),
    Exp(RefCell<Weak<Exp>>),
}

#[derive(Debug,Clone)]
pub struct Variable<T> {
    pub data:Rc<RefCell<Vec<T>>>,
    pub grad:Rc<RefCell<Vec<T>>>,
    pub creator:Option<Rc<RefCell<GradientFunction>>>,
}

impl <T> Variable<T> {
    fn new(value:Rc<RefCell<Vec<T>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
        Variable {
            data:value.clone(),
            grad:Rc::new(RefCell::new(Vec::with_capacity(0))),
            creator:None,
        }))
    }

    fn set_creator(&mut self, func: &Rc<RefCell<GradientFunction>>) {
        self.creator = Some(func.clone());
    }
}

trait Function<U> {
    fn call(&mut self,input:&Rc<RefCell<Variable<U>>>) -> Rc<RefCell<Variable<U>>> {
        let output = self.forward(input);
        
        self.update_input(input);
        self.update_ouput(&output);

        let gf = self.get_creator();
        output.borrow_mut().set_creator(&gf);

        output.clone()
    }

    fn forward(&self,x:&Rc<RefCell<Variable<U>>>) -> Rc<RefCell<Variable<U>>> {
        unimplemented!("forward not Implemented");
    }

    fn backward(&self,gy:&Rc<RefCell<Vec<U>>>) -> Vec<U> {
        unimplemented!("backward not Implemented");
    }

    fn update_input(&mut self, input: &Rc<RefCell<Variable<U>>>) {
        unimplemented!("update_input not Implemented");
    }

    fn update_ouput(&mut self,output:&Rc<RefCell<Variable<U>>>) {
        unimplemented!("update_ouput not Implemented");
    }

    fn get_creator(&mut self) -> Rc<RefCell<GradientFunction>> {
        unimplemented!("get_creator not Implemented");
    }
}

#[derive(Debug,Clone)]
struct Square {
    input:Rc<RefCell<Variable<f64>>>,
    output:Rc<RefCell<Variable<f64>>>,
}

impl Function<f64> for Square {
    fn forward(&self,x:&Rc<RefCell<Variable<f64>>>) -> Rc<RefCell<Variable<f64>>> {
        let nd_x = Array1::from_vec(x.borrow().data.borrow().clone());
        let x_squared = nd_x.map(|nd| nd.powi(2));
        Rc::new(RefCell::new(
        Variable {
            data:Rc::new(RefCell::new(x_squared.into_raw_vec())),
            grad:Rc::new(RefCell::new(Vec::<f64>::with_capacity(0))),
            creator:None,
        }))
    }

    fn backward(&self,gy:&Rc<RefCell<Vec<f64>>>) -> Vec<f64> {
        let nd_input = Array1::from_vec(self.input.borrow().data.borrow().clone());
        let nd_gy = Array1::from_vec(gy.borrow().clone());

        let nd_gx = 2.0*&nd_input*&nd_gy;

        nd_gx.into_raw_vec()
    }

    fn update_input(&mut self, input: &Rc<RefCell<Variable<f64>>>) {
        self.input = input.clone();
    }

    fn update_ouput(&mut self,output:&Rc<RefCell<Variable<f64>>>) {
        self.output = output.clone();
    }

    fn get_creator(&mut self) -> Rc<RefCell<GradientFunction>> {
        let obj = Rc::new(self.clone());
        let x = Rc::downgrade(&obj);
        Rc::new(RefCell::new(GradientFunction::Square(RefCell::new(x))))
    }
}

#[derive(Debug,Clone)]
struct Exp {
    input:Rc<RefCell<Variable<f64>>>,
    output:Rc<RefCell<Variable<f64>>>,
}

impl Function<f64> for Exp {
    fn forward(&self,x:&Rc<RefCell<Variable<f64>>>) -> Rc<RefCell<Variable<f64>>> {
        let nd_x = Array1::from_vec(x.borrow().data.borrow().clone());
        let nd_exp = nd_x.map(|nd| nd.exp());
        Rc::new(RefCell::new(
        Variable { 
            data:Rc::new(RefCell::new(nd_exp.into_raw_vec())),
            grad:Rc::new(RefCell::new(Vec::<f64>::with_capacity(0))),
            creator:None,
        }))
    }

    fn backward(&self,gy:&Rc<RefCell<Vec<f64>>>) -> Vec<f64> {
        let nd_input = Array1::from_vec(self.input.borrow().data.borrow().clone());
        let nd_gy = Array1::from_vec(gy.borrow().clone());

        let nd_exp = nd_input.map(|nd| nd.exp());
        let nd_gx = &nd_exp*&nd_gy;

        nd_gx.into_raw_vec()
    }

    fn update_input(&mut self, input: &Rc<RefCell<Variable<f64>>>) {
        self.input = input.clone();
    }

    fn update_ouput(&mut self,output:&Rc<RefCell<Variable<f64>>>) {
        self.output = output.clone();
    }

    fn get_creator(&mut self) -> Rc<RefCell<GradientFunction>> {
        let obj = Rc::new(self.clone());
        let x = Rc::downgrade(&obj);
        Rc::new(RefCell::new(GradientFunction::Exp(RefCell::new(x))))
    }
}



fn main() {
    let A = Rc::new(RefCell::new(Square {
        input:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))), 
        output:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))),
    }));

    let B = Rc::new(RefCell::new(Exp {
        input:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))), 
        output:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))),
    }));

    let C = Rc::new(RefCell::new(Square {
        input:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))), 
        output:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))),
    }));

    let nd_x = array![[0.5]];
    println!("nd_x {:?}",nd_x);
    let mut x = Variable::new(Rc::new(RefCell::new(nd_x.into_raw_vec())));
    println!("x {:?}",x);

    let mut a = A.borrow_mut().call(&x);
    let mut b = B.borrow_mut().call(&a);
    let mut y = C.borrow_mut().call(&b);
    println!("A {:?}",A.borrow());
    println!("B {:?}",B.borrow());
    println!("C {:?}",C.borrow());
    println!("y {:?}",y.borrow());
    println!("y.data {:?}",y.borrow().data.borrow());

    let nd_y_grad = array![[1.0]];
    println!("nd_y_grad {:?}",nd_y_grad);
    let y_grad_vec:Vec<f64> = nd_y_grad.into_raw_vec();
    println!("y_grad_vec {:?}",y_grad_vec);

    *y.borrow().grad.borrow_mut() = y_grad_vec;
    *b.borrow().grad.borrow_mut() = C.borrow_mut().backward(&y.borrow().grad);
    *a.borrow().grad.borrow_mut() = B.borrow_mut().backward(&b.borrow().grad);
    *x.borrow().grad.borrow_mut() = A.borrow_mut().backward(&a.borrow().grad);
    println!("x {:?}",x);
    if let Some(creator_rc) = &b.clone().borrow().creator {
        let creator = creator_rc.borrow();
        match &*creator {
            GradientFunction::Square(_) => println!("b Creator is Square"),
            GradientFunction::Exp(_) => println!("b Creator is Exp"),
        }
    } else {
        println!("No creator");
    }
}