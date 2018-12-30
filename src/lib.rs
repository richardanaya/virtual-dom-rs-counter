#![feature(unrestricted_attribute_tokens)]
#![feature(custom_attribute)]
#[macro_use]
extern crate virtual_dom_rs;
use crate::actions::AppAction;
use crate::reducers::app::AppState;
use crate::store::Store;
use crate::containers::counter_container::CounterContainer;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use std::cell::RefCell;

mod components;
mod containers;
mod reducers;
mod store;
mod virtual_dom_renderer;
mod actions;

// Create a store and put our initial state in it
thread_local!(static STORE : RefCell<Store<AppState, AppAction>> = RefCell::new(Store::new(AppState::new())));

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    log("hey");
    // Let's first get the body since this is going to be our root node
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = Element::from(document.body().unwrap());

    // This is my root component
    let todo_app = CounterContainer::new();

    // create our renderer
    // we need a refcell to mutate when state changes below
    let renderer = RefCell::new(virtual_dom_renderer::VirtualDomRenderer::new(body));

    // Render first time
    renderer.borrow_mut().render(&mut todo_app.render());

    STORE.with(|store| {
        // Add a listener to listen for state changes
        store.borrow_mut().add_listener(Box::new(move ||{
            log("new state");
            // Rerender everything again
            renderer.borrow_mut().render(&mut todo_app.render());
        }))
    });

    Ok(())
}
