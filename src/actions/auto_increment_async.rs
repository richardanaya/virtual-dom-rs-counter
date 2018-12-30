use crate::actions::AppAction;
use crate::reducers::app::AppState;
use std::rc::Rc;
use virtual_dom_rs::JsCast;
use wasm_bindgen::prelude::*;

pub fn auto_increment_async(_state: Rc<AppState>, dispatch: Rc<Fn(AppAction)>) -> Box<Fn()> {
    let async_dispatch = dispatch.clone();
    Box::new(move || {
        let timer_dispatch = async_dispatch.clone();
        let window = web_sys::window().unwrap();
        let a = Closure::wrap(Box::new(move || {
            timer_dispatch(AppAction::Increment);
        }) as Box<dyn FnMut()>);
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                a.as_ref().unchecked_ref(),
                1000,
            )
            .unwrap();
        a.forget();
    })
}
