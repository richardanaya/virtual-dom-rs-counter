use crate::log;
use std::rc::Rc;
use virtual_dom_rs::VirtualNode;

pub struct Counter {}
pub struct CounterProps {
    pub count: i32,
    pub increment: Box<Fn()>,
    pub decrement: Box<Fn()>,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {}
    }

    pub fn render(&self, props: Rc<CounterProps>) -> VirtualNode {
        // we clone ref to our props if we have more than event handler
        // otherwise we move it twice
        let p2 = props.clone();
        log("rendered");
        html! {
            <div>
                {format!("{}",props.count)}
                <div class="button",
                    !onclick=move |_ev| {
                        log("event happened");
                        (props.increment)()
                     },
                >
                    { "+" }
                </div>
                <div class="button",
                    !onclick=move |_ev| { (p2.decrement)() },
                >
                    { "-" }
                </div>
            </div>
        }
    }
}
