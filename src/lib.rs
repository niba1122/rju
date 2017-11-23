//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
//

#[macro_use]
extern crate stdweb;
use stdweb::web::{
    IEventTarget,
    IElement,
    IHtmlElement,
    INode,
    HtmlElement,
    Element,
    document,
    window
};

use stdweb::web::event::{
    ClickEvent
};


extern crate libc;

use std::fmt;
pub use std::sync::Mutex;
pub use std::sync::Arc;

#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref COMPONENTS: Mutex<HashMap<i32, Arc<Mutex<Component>>>> = Mutex::new(HashMap::new());
}

fn get_component(component_id: i32) -> Arc<Mutex<Component>> {
    Arc::clone(COMPONENTS.lock().unwrap().get_mut(&component_id).unwrap())
}

pub struct VirtualDOM {
    name: String,
    dom_type: DOMType,
    children: Vec<VirtualDOM>,
    attributes: Vec<Attribute>,
}

pub enum DOMType {
    Element(&'static str),
    Text(String),
    Comment,
    Component,
}

pub enum Attribute {
    String {
        name: &'static str,
        value: String
    },
    bool {
        name: &'static str,
        value: bool
    },
    EventHandler(fn(Arc<Mutex<Component>>))
    // EventHandler(Box<Fn() + 'static>)
}

impl fmt::Display for VirtualDOM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut children_string = String::new();
        for child in self.children.iter() {
            children_string.push_str(&format!("{}", child))
        }
        write!(f, "[{}({}){}]", self.name.to_string(), "", children_string)
    }
}

impl fmt::Display for DOMType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DOMType::Element(ref name) => {
                write!(f, "{}", name)
            },
            DOMType::Text(ref string) => {
                write!(f, "{}", string)
            },
            DOMType::Component => {
                write!(f, "component")
            }
            DOMType::Comment => {
                write!(f, "comment")
            }
        }
    }
}

pub struct Component {
    pub render: fn(Arc<Mutex<Component>>) -> VirtualDOM,
    pub state: i32
}
impl Component {
    pub fn update(&self) {
        let virtual_dom = (self.render)(get_component(1));
        let root_dom = document().get_element_by_id("test").unwrap();
        root_dom.set_text_content("");
        Renderer::render_dom(&root_dom, &virtual_dom, 1)
    }
    pub fn set_state(&mut self, state: i32) {
        self.state = state;
    }
}

pub struct Renderer;
impl Renderer {
    pub fn render(dom_id: &str, factory: fn() -> Component) {
        stdweb::initialize();

        // COMPONENTS.lock().unwrap().insert(1, factory());

        // let root_dom = document().get_element_by_id(dom_id).unwrap();
        // let virtual_dom = (COMPONENTS.lock().unwrap().get(&1).unwrap().render)(1);

        COMPONENTS.lock().unwrap().insert(1, Arc::new(Mutex::new(factory())));
        let mut component = get_component(1);
        let root_dom = document().get_element_by_id(dom_id).unwrap();
        let virtual_dom = (component.lock().unwrap().render)(get_component(1));

        Renderer::render_dom(&root_dom, &virtual_dom, 1);
        stdweb::event_loop();
    }

    pub fn render_dom(parent_dom: &Element, virtual_dom: &VirtualDOM, component_id: i32) {
        match virtual_dom.dom_type {
            DOMType::Element(ref name) => {
                let new_dom = document().create_element(&virtual_dom.name);
                for attribute in virtual_dom.attributes.iter() {
                    match *attribute {
                        Attribute::String { name, ref value } => {
                            new_dom.class_list().add(value)
                        },
                        Attribute::bool { name, ref value } => {
                        },
                        Attribute::EventHandler(callback) => {
                            new_dom.add_event_listener(move |_: ClickEvent| {
                                callback(get_component(component_id));
                            });
                        }
                    }
                }
                parent_dom.append_child(&new_dom);
                for child in virtual_dom.children.iter() {
                   Renderer::render_dom(&new_dom, child, component_id);
                }
            }
            DOMType::Text(ref string) => {
                let new_dom = document().create_text_node(string);
                parent_dom.append_child(&new_dom);
            }
            DOMType::Component => {}
            DOMType::Comment => {}
        };
    }
}

pub fn h(dom_type: DOMType, children: Vec<VirtualDOM>, attributes: Vec<Attribute>) -> VirtualDOM {
    return VirtualDOM {
        name: dom_type.to_string(),
        dom_type: dom_type,
        children: children,
        attributes: attributes
    };
}