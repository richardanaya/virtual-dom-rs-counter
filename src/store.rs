use std::marker::PhantomData;
use virtual_dom_rs::VirtualNode;
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::Arc;

pub trait Reducer<P> {
    fn reduce(&self, action: P) -> Option<Self>
    where
        Self: std::marker::Sized;
}

type Listener = Fn() + Send + Sync + 'static;

pub struct Store<T, P> {
    state: T,
    listeners: Vec<Arc<Box<Listener>>>,
    _p: PhantomData<P>,
}

impl<T: Reducer<P>, P> Store<T, P> {
    pub fn new(initial_value: T) -> Store<T, P> {
        Store {
            state: initial_value,
            _p: PhantomData,
            listeners: vec![]
        }
    }

    pub fn connect(store_mutex:&'static Mutex<Store<T, P>>, handler: Box<Fn(&T,Rc<Fn(P)>) -> VirtualNode>) -> VirtualNode {
        let store = store_mutex.lock().unwrap();
        handler(&store.state,Rc::new(move |p|{
            let mut store = store_mutex.lock().unwrap();
            store.dispatch(p)
        }))
    }

    pub fn dispatch(&mut self, action: P) {
        let t = self.state.reduce(action);
        if let Some(new_state) = t {
            self.state = new_state;
        }
    }

    pub fn add_listener<F>(&mut self,listener:Box<Listener>) {
        self.listeners.push(Arc::new(listener))
    }
}
