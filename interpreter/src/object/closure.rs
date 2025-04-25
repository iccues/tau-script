use crate::object_box::ObjectBox;

use super::{tuple::Tuple, Object, ObjectTrait};

#[derive(Debug)]
pub struct Closure {
    func: Object,
    env: Vec<Option<Object>>,
}

impl Closure {
    pub fn new(func: Object, env: Vec<Option<Object>>) -> Object {
        ObjectBox::new(Self { func, env })
    }

    pub fn curry_opt(&mut self, args: &[Option<Object>]) {
        let mut i = 0;
        for arg in args {
            while i < self.env.len() && self.env[i].is_some() {
                i += 1;
            }

            if i < self.env.len() {
                self.env[i] = arg.clone();
            } else {
                self.env.push(arg.clone());
            }
            i += 1;
        }
    }

    pub fn curry(&self, args: &[Object]) -> Option<Vec<Object>> {
        let mut env = self.env.clone();
        let mut i = 0;

        for arg in args {
            while i < self.env.len() && self.env[i].is_some() {
                i += 1;
            }

            if i < self.env.len() {
                env[i] = Some(arg.clone());
            } else {
                env.push(Some(arg.clone()));
            }
            i += 1;
        }

        env.into_iter().collect()
    }
}

impl ObjectTrait for Closure {
    fn to_string_row(&self) -> String {
        format!("Closere[ func: {} ]", self.func.to_string_row())
    }

    fn call(&self, input: Object) -> Object {
        let env: Option<Vec<Object>> = self.curry(input.downcast::<Tuple>().unwrap().deconst());
        let input = Tuple::new(env.unwrap());
        self.func.call(input)
    }
}