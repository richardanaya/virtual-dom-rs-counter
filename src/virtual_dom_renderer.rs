use virtual_dom_rs::VirtualNode;
use web_sys::Element;

// This object will help us store the previous dom and render to a target element
pub struct VirtualDomRenderer {
    root_element: Element,
    previous_vdom: Option<VirtualNode>,
}

impl VirtualDomRenderer {
    pub fn new(root_element: Element) -> VirtualDomRenderer {
        VirtualDomRenderer {
            root_element: root_element,
            previous_vdom: None,
        }
    }

    pub fn render(&mut self, new_vdom: &mut VirtualNode) {
        if let Some(p_vd) = &self.previous_vdom {
            // If its not the first time, calculate the DOM dif and apply to root dom contents
            let patches = virtual_dom_rs::diff(&p_vd, new_vdom);
            virtual_dom_rs::patch(self.root_element.clone(), &patches);
        } else {
            // If its the first time just set the contents of the DOM to string
            self.root_element.set_inner_html(&new_vdom.to_string());
        }
    }
}
