use object_core::prelude::*;
use object_ext::object_trait_ext::ObjectTraitExt;
use object_ext::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Closure {
    func: Object,
    context: Vec<Object>,
    times: usize,
}

impl ObjectTrait for Closure {}

impl ObjectTraitExt for Closure {
    const CALLABLE: bool = true;
    fn call(this: Object<Self>, input: Object) -> Object {
        let mut closure = (**this).clone();
        closure.context.push(input);
        if closure.context.len() == closure.times {
            closure.func.call(Tuple::new(closure.context))
        } else {
            closure.from_data()
        }
    }
}

impl Closure {
    pub fn new(func: Object, times: usize) -> Object<Closure> {
        Closure {
            func,
            context: vec![],
            times,
        }.from_data()
    }
}
