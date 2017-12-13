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
extern crate rand;

use std::fmt;
pub use std::sync::Mutex;
pub use std::sync::Arc;
pub use std::any::Any;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use rand::Rng;

#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref COMPONENTS: Mutex<HashMap<u64, Arc<Mutex<Component>>>> = Mutex::new(HashMap::new());
}

fn get_component(component_id: u64) -> Arc<Mutex<Component>> {
    Arc::clone(COMPONENTS.lock().unwrap().get_mut(&component_id).unwrap())
}

pub struct VirtualDOM {
    name: String,
    dom_type: DOMType,
    children: Vec<VirtualDOM>,
    attributes: Vec<Attribute>,
    dom_id: Option<u64>
}

pub enum DOMType {
    Element(&'static str),
    Text(String),
    Comment,
    Component(fn() -> InitialComponent)
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
            DOMType::Component(factory) => {
                write!(f, "component")
            }
            DOMType::Comment => {
                write!(f, "comment")
            }
        }
    }
}

pub struct InitialComponent {
    pub render: fn(Arc<Mutex<Component>>) -> VirtualDOM,
    pub state: Arc<Mutex<State>>,
}

pub struct Component {
    id: u64,
    pub render: fn(Arc<Mutex<Component>>) -> VirtualDOM,
    pub state: Arc<Mutex<State>>,
    dom_id: String
}
impl Component {
    pub fn update(&self) {
        let mut virtual_dom = (self.render)(get_component(self.id));
        let root_dom = document().get_element_by_id(&self.dom_id).unwrap();
        root_dom.set_text_content("");
        Renderer::render_dom(&root_dom, &mut virtual_dom, self.id)
    }
    pub fn set_state(&mut self, state: Arc<Mutex<State>>) {
        self.state = state;
    }
}

pub trait State : Send {
    fn as_any(&mut self) -> &mut Any;
}

pub struct Renderer;
impl Renderer {
    pub fn render(dom_id: &str, factory: fn() -> InitialComponent) {
        stdweb::initialize();

        Renderer::_render(dom_id, factory);

        stdweb::event_loop();
    }

    fn _render(dom_id: &str, factory: fn() -> InitialComponent) {
        let id = Renderer::generate_id();
        let initial_component = factory();
        let component = Component {
            id: id,
            render: initial_component.render,
            state: initial_component.state,
            dom_id: dom_id.to_string()
        };
        let component_ref = Arc::new(Mutex::new(component));
        let root_dom = document().get_element_by_id(dom_id).unwrap();
        let mut virtual_dom = (component_ref.lock().unwrap().render)(component_ref.clone());

        COMPONENTS.lock().unwrap().insert(id, component_ref.clone());
        Renderer::render_dom(&root_dom, &mut virtual_dom, id);
    }

    pub fn render_dom(parent_dom: &Element, virtual_dom: &mut VirtualDOM, component_id: u64) {
        match virtual_dom.dom_type {
            DOMType::Element(ref name) => {
                let new_dom = document().create_element(&virtual_dom.name);
                let dom_id = Renderer::generate_id();
                virtual_dom.dom_id = Some(dom_id);
                js! {
                    @{&new_dom}.id = @{dom_id.to_string()};
                }
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
                for child in &mut virtual_dom.children {
                   Renderer::render_dom(&new_dom, &mut *child, component_id);
                }
            }
            DOMType::Text(ref string) => {
                let new_dom = document().create_text_node(string);
                parent_dom.append_child(&new_dom);
            }
            DOMType::Component(ref factory) => {
                let new_dom = document().create_element(&virtual_dom.name);
                let dom_id = Renderer::generate_id().to_string();
                js! {
                    @{&new_dom}.id = @{&dom_id};
                }
                parent_dom.append_child(&new_dom);
                Renderer::_render(&dom_id, *factory);
            }
            DOMType::Comment => {}
        };
    }

    fn generate_id() -> u64 {
        let mut rng = rand::thread_rng();
        let mut hasher = DefaultHasher::new();
        let random_u64 = rng.gen::<u64>();
        random_u64.hash(&mut hasher);
        hasher.finish()
    }
}

pub fn h(dom_type: DOMType, children: Vec<VirtualDOM>, attributes: Vec<Attribute>) -> VirtualDOM {
    return VirtualDOM {
        name: dom_type.to_string(),
        dom_type: dom_type,
        children: children,
        attributes: attributes,
        dom_id: None
    };
}