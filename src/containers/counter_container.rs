use super::super::components::Counter;
use super::super::STORE;
use virtual_dom_rs::VirtualNode;

pub struct CounterContainer {}

impl CounterContainer {
    pub fn new() -> CounterContainer {
        CounterContainer {}
    }

    pub fn render(&self) -> VirtualNode {
        let store = STORE.lock().unwrap();
        store.connect(|state| {
            Counter::new().render(state.count)
        })
    }
}
