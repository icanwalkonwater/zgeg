use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

mod nodes;
pub use nodes::*;
mod parser;

struct Ast<T> {
    i: usize,
    _pd: PhantomData<T>,
}

struct AstManager {
    storage: HashMap<TypeId, Box<dyn Any>>,
}

impl AstManager {
    fn get<T: 'static>(&self, handle: Ast<T>) -> &T {
        let v = self.storage.get(&TypeId::of::<T>()).unwrap();
        v.downcast_ref::<Vec<T>>().unwrap().get(handle.i).unwrap()
    }

    fn push<T: 'static>(&mut self, node: T) -> Ast<T> {
        let v = self
            .storage
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(Vec::<T>::new()));
        let vv = v.downcast_mut::<Vec<T>>().unwrap();
        let i = vv.len();
        vv.push(node);

        Ast {
            i,
            _pd: Default::default(),
        }
    }
}
