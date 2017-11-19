#![feature(proc_macro)]
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let text = input.to_string();
    html_parser::parse(text).parse().unwrap()
}

extern crate xml5ever;
mod html_parser {
    use std::str::FromStr;
    use std::cell::RefCell;

    use xml5ever::tendril::{Tendril, TendrilSink};
    use xml5ever::driver::parse_document;
    use xml5ever::rcdom::{Handle, NodeData, RcDom};
    use xml5ever::Attribute;

    fn generate_child_elements_string(children: &RefCell<Vec<Handle>>) -> String {
        let mut output: String = String::new();
        output.push_str("vec![");
        for (i, child) in children.borrow().iter().enumerate() {
            if i > 0 {
                output.push_str(",");
            }
            let child_str = generate_create_element_string(0, child.clone());
            output.push_str(&child_str);
        }
        output.push_str("]");
        format!("{}", output)
    }

    fn generate_type_string(data: &NodeData) -> String {
        match *data {
            NodeData::Document => {
                "DOMType::Element(\"document\")".to_string()
            }

            NodeData::Doctype {
                ref name,
                ref public_id,
                ref system_id,
            } => {
                "DOMType::Element(\"doctype\")".to_string()
            }

            NodeData::Text { ref contents } => {
                let contents_string = contents.borrow().to_string();
                let mut embed = false;
                let mut new_string = String::new();
                let mut variables: Vec<String> = vec![];
                for c in contents_string.chars() {
                    if c == '}' {
                        embed = false
                    }
                    if embed {
                        variables.last_mut().unwrap().push(c);
                    } else {
                        new_string.push(c);
                    }
                    if c == '{' {
                        embed = true;
                        variables.push(String::new());
                    }               
                }
                println!("DOMType::Text(format!(\"{}\",{}))", new_string, variables.join(","));
                format!("DOMType::Text(format!(\"{}\",{}))", new_string, variables.join(","))
            }

            NodeData::Comment { ref contents } => {
                "DOMType::Comment".to_string()
            }

            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                format!("DOMType::Element(\"{}\")", name.local.to_string())
            }

            NodeData::ProcessingInstruction { .. } => unreachable!(),
        }
    }
    fn generate_attributes_string(data: &NodeData) -> String {
        match *data {
            NodeData::Document => {
                "vec![]".to_string()
            }

            NodeData::Doctype {
                ref name,
                ref public_id,
                ref system_id,
            } => {
                "vec![]".to_string()
            }

            NodeData::Text { ref contents } => {
                "vec![]".to_string()
            }

            NodeData::Comment { ref contents } => {
                "vec![]".to_string()
            }

            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                let mut output: String = String::new();
                output.push_str("vec![");
                for (i, attr) in attrs.borrow().iter().enumerate() {
                    if i > 0 {
                        output.push_str(",");
                    }
                    let Attribute {
                        ref name,
                        ref value,
                    } = *attr;

                    match name.prefix {
                        Some(ref prefix) => {
                            if prefix.eq("bind") {
                                output.push_str(&format!("Attribute::String{{name:\"{}\",value:{}}}", name.local, value));
                            } else if prefix.eq("on") {
                                output.push_str(&format!("Attribute::EventHandler({})", value));
                            } else {
                                output.push_str(&format!("Attribute::String{{name:\"{}:{}\",value:\"{}\".to_string()}}", prefix, name.local, value));
                            }
                        }
                        None => {
                            output.push_str(&format!("Attribute::String{{name:\"{}\",value:\"{}\"}}", name.local, value));
                        }
                    }
                }
                output.push_str("]");
                format!("{}", output)
            }

            NodeData::ProcessingInstruction { .. } => unreachable!(),
        }

    }

    fn generate_create_element_string<'a>(_indent: usize, handle: Handle) -> String {
        let node = handle;
        let children: String = generate_child_elements_string(&node.children);
        let dom_type: String = generate_type_string(&node.data);
        let attributes: String = generate_attributes_string(&node.data);

        format!(
            "h({}, {}, {})",
            dom_type,
            children,
            attributes
        )
    }

    pub fn parse<'a>(html: String) -> String {
        let str = Tendril::from_str(&*html).unwrap();
        let mut parser = parse_document(RcDom::default(), Default::default());
        parser.process(str);
        let dom = parser.finish();

        let render = generate_create_element_string(0, dom.document);
        println!("{}", render);
        render
    }
}
