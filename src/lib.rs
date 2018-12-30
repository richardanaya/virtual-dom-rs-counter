#![feature(unrestricted_attribute_tokens)]
#![feature(custom_attribute)]
#[macro_use]
extern crate virtual_dom_rs;
use crate::actions::AppAction;
use crate::reducers::AppState;
use crate::store::Store;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use std::cell::RefCell;

mod components;
mod containers;
mod reducers;
mod store;
mod virtual_dom_renderer;
mod actions;

// Create a store
thread_local!(static STORE : RefCell<Store<AppState, AppAction>> = RefCell::new(Store::new(AppState::new())));

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Let's first get the body since this is going to be our root node
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = Element::from(document.body().unwrap());

    // This is my root component
    let todo_app = containers::CounterContainer::new();

    // create our renderer
    let renderer = RefCell::new(virtual_dom_renderer::VirtualDomRenderer::new(body));
    renderer.borrow_mut().render(&mut todo_app.render());

    STORE.with(|store| {
        store.borrow_mut().add_listener(Box::new(move ||{
            renderer.borrow_mut().render(&mut todo_app.render());
        }))
    });

    Ok(())
}
