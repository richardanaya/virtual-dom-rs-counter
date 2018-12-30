use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use virtual_dom_rs::VirtualNode;

pub trait Reducer<P> {
    fn reduce(&self, action: P) -> Option<Self>
    where
        Self: std::marker::Sized;
}

pub struct Store<T, P> {
    state: T,
    listeners: Vec<Box<Fn()>>,
    _p: PhantomData<P>,
}

impl<T: Reducer<P>, P> Store<T, P> {
    pub fn new(initial_value: T) -> Store<T, P> {
        Store {
            state: initial_value,
            _p: PhantomData,
            listeners: vec![],
        }
    }

    pub fn connect(
        store_thread_key: &'static std::thread::LocalKey<RefCell<Store<T, P>>>,
        handler: Box<Fn(&T, Rc<Fn(P)>) -> VirtualNode>,
    ) -> VirtualNode {
        store_thread_key.with(|store| {
            handler(
                &store.borrow().state,
                Rc::new(move |p| store_thread_key.with(|store| store.borrow_mut().dispatch(p))),
            )
        })
    }

    pub fn dispatch(&mut self, action: P) {
        let t = self.state.reduce(action);
        if let Some(new_state) = t {
            self.state = new_state;
            for listener in self.listeners.iter() {
                listener();
            }
        }
    }

    pub fn add_listener(&mut self, listener: Box<Fn()>) {
        self.listeners.push(listener)
    }
}
