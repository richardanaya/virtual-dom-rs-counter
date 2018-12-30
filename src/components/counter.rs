use virtual_dom_rs::VirtualNode;

pub struct Counter {}

impl Counter {
    pub fn new() -> Counter {
        Counter {}
    }

    pub fn render(&self, count:i32) -> VirtualNode {
        html! {
            <div>
                {format!("{}",count)}
                <button>{"+"}</button>
                <button>{"-"}</button>
            </div>
        }
    }
}
