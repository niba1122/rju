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
use std::rc::{Rc};

use std::any::Any;

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref COMPONENTS: Mutex<HashMap<&'static str, Component>> = Mutex::new(HashMap::new());
    // static ref HASHMAP: Vec<Component> = {
    //     let mut m = vec![];
    //     m
    // };
}

pub struct VirtualDOM {
    name: String,
    dom_type: DOMType,
    children: Vec<VirtualDOM>,
    // attributes: Vec<(String, String)>,
    // attributes: Vec<&'static Attribute>
    attributes: Vec<Attribute>,
}

pub enum DOMType {
    Element(&'static str),
    Text(&'static str),
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
    EventHandler(fn(i32))
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
    pub render: fn(i32) -> VirtualDOM,
}
impl Component {
    fn update(&self) {
        (self.render)(10);
    }
}

pub struct Renderer;
impl Renderer {
    pub fn render(dom_id: &str, factory: fn() -> Component) {
        stdweb::initialize();
        let component = factory();
        let virtual_dom = (component.render)(10);

        // let mut hashmap = COMPONENTS.;
        // hashmap.insert("hoge", component);
        // COMPONENTS.insert("hoge", component);
        COMPONENTS.lock().unwrap().insert("hoge", component);

        let root_dom = document().get_element_by_id(dom_id).unwrap();
        Renderer::render_dom(&root_dom, &virtual_dom, COMPONENTS.lock().unwrap().get("hoge").unwrap());
        stdweb::event_loop();
    }

    pub fn render_dom(parent_dom: &Element, virtual_dom: &VirtualDOM, component: &Component) {
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

                            // let ptr = Renderer::to_unsafe(component) as *const T;
                            new_dom.add_event_listener(move |_: ClickEvent| {
                                // let hoge = unsafe { &*ptr };
                                callback(10);
                            });
                        }
                    }
                }
                parent_dom.append_child(&new_dom);
                for child in virtual_dom.children.iter() {
                   Renderer::render_dom(&new_dom, child, component);
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