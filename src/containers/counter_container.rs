use crate::actions::auto_increment_async::auto_increment_async;
use crate::actions::AppAction;
use crate::components::counter::Counter;
use crate::components::counter::CounterProps;
use crate::log;
use crate::reducers::app::Selectors;
use crate::store::Store;
use crate::STORE;
use std::rc::Rc;
use virtual_dom_rs::VirtualNode;

pub struct CounterContainer {}

impl CounterContainer {
    pub fn new() -> CounterContainer {
        CounterContainer {}
    }

    pub fn render(&self) -> VirtualNode {
        log("rendering container");
        Store::connect(
            &STORE,
            Box::new(|state, dispatch| {
                log("got state");
                // We need to clone a state/dispatcher if we have more than one handler
                // otherwise we move references twice
                let d2 = dispatch.clone();
                let d3 = dispatch.clone();
                let s2 = state.clone();
                Counter::new().render(Rc::new(CounterProps {
                    count: Selectors::get_count(state),
                    increment: Box::new(move || {
                        dispatch(AppAction::Increment);
                    }),
                    decrement: Box::new(move || {
                        d2(AppAction::Decrement);
                    }),
                    auto: auto_increment_async(s2, d3),
                }))
            }),
        )
    }
}
