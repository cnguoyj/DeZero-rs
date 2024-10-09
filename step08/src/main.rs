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
pub struct Variable {
    pub data:Rc<RefCell<Vec<f64>>>,
    pub grad:Rc<RefCell<Vec<f64>>>,
    pub creator:Option<Rc<RefCell<GradientFunction>>>,
}

impl Variable {
    fn new(value:Rc<RefCell<Vec<f64>>>) -> Rc<RefCell<Self>> {
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
    
    //  loop
    fn backward(&mut self) {
        let var_rc = Rc::new(RefCell::new(self.to_owned()));
        let mut stack = vec![var_rc]; 

        while let Some(current_var) = stack.pop() {
            let current = current_var.borrow_mut();
            if let Some(creator_rc) = &current.creator {
                let creator = creator_rc.borrow();
                match &*creator {
                    GradientFunction::Square(square_ref) => {
                        let square_rc = square_ref.borrow().upgrade();

                        match square_rc {
                            Some(ref square) => {
                                let x = &square.input;
                                let gy = current.grad.clone();
                                *x.borrow().grad.borrow_mut() = square.backward(&gy);
                                let var = x.borrow_mut().to_owned();
                                let var_rc = Rc::new(RefCell::new(var));
                                stack.push(var_rc); 
                            },
                            None => {
                                // println!("Variable square_rc Creator None");
                            },
                        }
                    },
                    GradientFunction::Exp(exp_ref) => {
                        let exp_rc = exp_ref.borrow().upgrade();

                        match exp_rc {
                            Some(ref exp) => {
                                let x = &exp.input;
                                let gy = current.grad.clone();
                                *x.borrow().grad.borrow_mut() = exp.backward(&gy);
                                let var = x.borrow_mut().to_owned();
                                let var_rc = Rc::new(RefCell::new(var));
                                stack.push(var_rc); 
                            },
                            None => {
                                // println!("Variable exp_rc Creator None");
                            },
                        }
                    }
                }
            }
            // 如果没有 creator，循环将继续处理其他变量
        }
    }
}

trait Function {
    fn call(&mut self,input:&Rc<RefCell<Variable>>) -> Rc<RefCell<Variable>> {
        let output = self.forward(input);
        
        self.update_input(input);
        self.update_ouput(&output);

        let gf = self.get_creator();
        output.borrow_mut().set_creator(&gf);

        output
    }

    fn forward(&self,x:&Rc<RefCell<Variable>>) -> Rc<RefCell<Variable>> {
        unimplemented!("forward not Implemented");
    }

    fn backward(&self,gy:&Rc<RefCell<Vec<f64>>>) -> Vec<f64> {
        unimplemented!("backward not Implemented");
    }

    fn update_input(&mut self, input: &Rc<RefCell<Variable>>) {
        unimplemented!("update_input not Implemented");
    }

    fn update_ouput(&mut self,output:&Rc<RefCell<Variable>>) {
        unimplemented!("update_ouput not Implemented");
    }

    fn get_creator(&mut self) -> Rc<RefCell<GradientFunction>> {
        unimplemented!("get_creator not Implemented");
    }
}

#[derive(Debug,Clone)]
struct Square {
    selfclone:Option<Rc<Square>>,
    input:Rc<RefCell<Variable>>,
    output:Rc<RefCell<Variable>>,
}

impl Function for Square {
    fn forward(&self,x:&Rc<RefCell<Variable>>) -> Rc<RefCell<Variable>> {
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

    fn update_input(&mut self, input: &Rc<RefCell<Variable>>) {
        self.input = input.clone();
    }

    fn update_ouput(&mut self,output:&Rc<RefCell<Variable>>) {
        self.output = output.clone();
    }

    fn get_creator(&mut self) -> Rc<RefCell<GradientFunction>> {
        let sc = self.clone();
        let obj = Rc::new(sc);
        self.selfclone = Some(obj.clone());
        let x = Rc::downgrade(&obj);
        let gf = GradientFunction::Square(RefCell::new(x));
        Rc::new(RefCell::new(gf))
    }
}

#[derive(Debug,Clone)]
struct Exp {
    selfclone:Option<Rc<Exp>>,
    input:Rc<RefCell<Variable>>,
    output:Rc<RefCell<Variable>>,
}

impl Function for Exp {
    fn forward(&self,x:&Rc<RefCell<Variable>>) -> Rc<RefCell<Variable>> {
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

    fn update_input(&mut self, input: &Rc<RefCell<Variable>>) {
        self.input = input.clone();
    }

    fn update_ouput(&mut self,output:&Rc<RefCell<Variable>>) {
        self.output = output.clone();
    }

    fn get_creator(&mut self) -> Rc<RefCell<GradientFunction>> {
        let sc = self.clone();
        let obj = Rc::new(sc);
        self.selfclone = Some(obj.clone());
        let x = Rc::downgrade(&obj);
        let gf = GradientFunction::Exp(RefCell::new(x));
        Rc::new(RefCell::new(gf))
    }
}

fn main() {
    let A = Rc::new(RefCell::new(Square {
        selfclone:None,
        input:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))), 
        output:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))),
    }));

    let B = Rc::new(RefCell::new(Exp {
        selfclone:None,
        input:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))), 
        output:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))),
    }));

    let C = Rc::new(RefCell::new(Square {
        selfclone:None,
        input:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))), 
        output:Variable::new(Rc::new(RefCell::new(Vec::<f64>::with_capacity(0)))),
    }));

    let nd_x = array![[0.5]];
    let x = Variable::new(Rc::new(RefCell::new(nd_x.into_raw_vec())));

    let a = A.borrow_mut().call(&x);
    let b = B.borrow_mut().call(&a);
    let y = C.borrow_mut().call(&b);
    println!("y.data {:?}",y.borrow().data.borrow());

    let nd_y_grad = array![[1.0]];
    println!("nd_y_grad {:?}",nd_y_grad);
    let y_grad_vec:Vec<f64> = nd_y_grad.into_raw_vec();
    println!("y_grad_vec {:?}",y_grad_vec);

    *y.borrow().grad.borrow_mut() = y_grad_vec;
    println!("y {:?}",y);
    y.borrow_mut().backward();
    println!("x {:?}",x);
}