use crate::object::{object::Object, object_trait::ObjectTrait};

use super::tuple::Tuple;

pub struct Closure {
    func: Object,
    context: Vec<Option<Object>>,
}

impl ObjectTrait for Closure {
    fn call_fn(this: Object, args: Object) -> Object {
        let this = this.get_data::<Closure>().unwrap();
        let args = args.get_data::<Tuple>().unwrap();

        let args = this.curry(&args.elements);
        this.func.call(Tuple::new(args.unwrap()))
    }
}

impl Closure {
    pub fn new(func: Object, context: Vec<Option<Object>>) -> Object {
        Self::from_data(Closure { func, context })
    }

    pub fn curry(&self, args: &[Object]) -> Option<Vec<Object>> {
        let mut context = self.context.clone();
        let mut i = 0;

        for arg in args {
            while i < self.context.len() && self.context[i].is_some() {
                i += 1;
            }

            if i < self.context.len() {
                context[i] = Some(arg.clone());
            } else {
                context.push(Some(arg.clone()));
            }
            i += 1;
        }

        context.into_iter().collect()
    }
}