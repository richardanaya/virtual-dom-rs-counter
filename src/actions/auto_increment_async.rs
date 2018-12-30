use crate::actions::AppAction;
use crate::reducers::app::AppState;
use std::rc::Rc;
use virtual_dom_rs::JsCast;
use wasm_bindgen::prelude::*;

// this is a simple asynchronous action that dispatches multiple times on a timer
pub fn auto_increment_async(_state: Rc<AppState>, dispatch: Rc<Fn(AppAction)>) -> Box<Fn()> {
    let async_dispatch = dispatch.clone();
    Box::new(move || {
        let timer_dispatch = async_dispatch.clone();
        // increment immediately
        timer_dispatch(AppAction::Increment);
        // let create a closure that gets called every 1000ms
        let window = web_sys::window().unwrap();
        let a = Closure::wrap(Box::new(move || {
            // increment again ...
            timer_dispatch(AppAction::Increment);
        }) as Box<dyn FnMut()>);
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                a.as_ref().unchecked_ref(),
                1000,
            )
            .unwrap();
        // this is forgotten so our closure doesn't disappear
        a.forget();
    })
}
