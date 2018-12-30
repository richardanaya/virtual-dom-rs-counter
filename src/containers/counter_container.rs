use crate::actions::AppAction;
use crate::components::counter::Counter;
use crate::components::counter::CounterProps;
use crate::store::Store;
use crate::STORE;
use std::rc::Rc;
use virtual_dom_rs::VirtualNode;
use crate::reducers::app::Selectors;

pub struct CounterContainer {}

impl CounterContainer {
    pub fn new() -> CounterContainer {
        CounterContainer {}
    }

    pub fn render(&self) -> VirtualNode {
        Store::connect(
            &STORE,
            Box::new(|state, dispatch| {
                // We need to clone a dispatcher if we have more than one handler
                // otherwise we move it twice
                let d2 = dispatch.clone();
                Counter::new().render(Rc::new(CounterProps {
                    count: Selectors::getCount(state),
                    increment: Box::new(move || {
                        dispatch(AppAction::Increment);
                    }),
                    decrement: Box::new(move || {
                        d2(AppAction::Decrement);
                    }),
                }))
            }),
        )
    }
}
