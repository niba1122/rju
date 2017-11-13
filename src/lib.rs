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
// use std::ffi::CString;

pub struct VirtualDOM {
    name: String,
    dom_type: DOMType,
    children: Vec<VirtualDOM>,
    // attributes: Vec<(String, String)>,
    attributes: Vec<Attribute>
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
    // EventHandler()
    EventHandler(fn())
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

pub struct Component<F> where F : Fn(&Component<F>) -> VirtualDOM {
    pub parent_dom_id: &'static str,
    pub render: F
}

impl <F>Component<F> where F : Fn(&Component<F>) -> VirtualDOM {
    pub fn update(&self) {
        //root_dom.children = vec![self.render()]
        let new_virtual_dom = (self.render)(self);
        Renderer::patch(self.parent_dom_id, new_virtual_dom);
    }
}

pub struct Renderer;
impl Renderer {
    pub fn patch(dom_id: &str, virtual_dom: VirtualDOM) {
        stdweb::initialize();

        let root_dom = document().get_element_by_id(dom_id).unwrap();
        Renderer::render_dom(&root_dom, &virtual_dom);
        stdweb::event_loop();
    }

    pub fn render_dom(parent_dom: &Element, virtual_dom: &VirtualDOM) {
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
                                callback();
                            });
                        }
                    }
                }
                parent_dom.append_child(&new_dom);
                for child in virtual_dom.children.iter() {
                   Renderer::render_dom(&new_dom, child);
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

// pub fn h(dom_type: DOMType, children: Vec<VirtualDOM>, attributes: Vec<(&str, &str)>) -> VirtualDOM {
pub fn h(dom_type: DOMType, children: Vec<VirtualDOM>, attributes: Vec<Attribute>) -> VirtualDOM {
    return VirtualDOM {
        name: dom_type.to_string(),
        dom_type: dom_type,
        children: children,
        attributes: attributes
    };
}

