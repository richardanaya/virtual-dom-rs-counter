use std::marker::PhantomData;
use virtual_dom_rs::VirtualNode;

pub trait Reducer<P> {
    fn reduce(&self, action: P) -> Option<Self>
    where
        Self: std::marker::Sized;
}

pub struct Store<T, P> {
    state: T,
    _p: PhantomData<P>,
}

impl<T: Reducer<P>, P> Store<T, P> {
    pub fn new(initial_value: T) -> Store<T, P> {
        Store {
            state: initial_value,
            _p: PhantomData,
        }
    }

    pub fn connect(&self, handler: fn(&T) -> VirtualNode) -> VirtualNode {
        handler(&self.state)
    }

    pub fn dispatch(&mut self, action: P) {
        let t = self.state.reduce(action);
        if let Some(new_state) = t {
            self.state = new_state;
        }
    }
}
