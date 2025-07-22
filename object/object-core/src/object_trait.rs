use std::{any::Any, fmt::Debug};

pub trait ObjectTrait: Any + Debug {}

impl ObjectTrait for () {}
