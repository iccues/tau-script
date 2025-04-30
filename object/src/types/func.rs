use crate::object::{object::Object, object_trait::ObjectTrait};

pub struct Func {
    func: Box<dyn Fn(Object) -> Object>,
}

impl ObjectTrait for Func {
    fn call_fn(this: Object, args: Object) -> Object {
        let func = &this.get_data_as::<Func>().func;
        func(args)
    }
}

impl Func {
    pub fn new<F>(func: F) -> Object
    where
        F: Fn(Object) -> Object + 'static,
    {
        Self::from_data(
            Func { func: Box::new(func) }
        )
    }
}
