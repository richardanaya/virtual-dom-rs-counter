use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread::LocalKey;
use std::cmp::Eq;

pub fn create_selector<B>(c:&'static LocalKey<RefCell<HashMap<Rc<B>,i32>>>,f:fn(Rc<B>)->i32) -> Box<Fn(Rc<B>)->i32> where B:Hash + Eq {
    Box::new(move|x:Rc<B>|{
        c.with(|cache|{
            if let Some(v) = cache.borrow().get(&x.clone()) {
                // cache is hit
                return *v
            }
            // no cache is hit
            let r = f(x.clone());
            cache.borrow_mut().insert(x,r);
            r
        })
    })
}
