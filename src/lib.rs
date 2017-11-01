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
  //use html5ever::parse_document;
  //use html5ever::rcdom::{NodeData, RcDom, Handle};
  //use html5ever::tendril::TendrilSink;
  //use html5ever::tendril::Tendril;
  use std::str::FromStr;
  use std::cell::{RefCell};
  use std::borrow::Cow;

  use xml5ever::tendril::{Tendril, TendrilSink};
  use xml5ever::driver::parse_document;
  use xml5ever::rcdom::{RcDom, NodeData, Handle};
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

  fn generate_attributes_string(attrs: &RefCell<Vec<Attribute>>) -> String {
      let mut output: String = String::new();
      output.push_str("vec![");
      for (i, attr) in attrs.borrow().iter().enumerate() {
          if i > 0 {
              output.push_str(",");
          }
         let Attribute { ref name, ref value } = *attr;
         output.push_str("\"");
         output.push_str(&*name.local);
         output.push_str("\", \"");
         output.push_str(value);
         output.push_str("\"");
         //println!("{},{}", &*name.local, value.to_string());
      }
      output.push_str("]");
      format!("{}", output)
  }

  fn generate_create_element_string<'a>(_indent: usize, handle: Handle) -> String {
      let node = handle;
      let tag_name: Cow<'a, str>;
      let attributes: Cow<'a, str>;
      match node.data {
          NodeData::Document
              => {
                  tag_name = Cow::Owned("document".to_string());
                  attributes = Cow::Owned(generate_attributes_string(&RefCell::new(vec![])));
                },

          NodeData::Doctype { ref name, ref public_id, ref system_id }
              => {
                    tag_name = Cow::Owned("doctype".to_string());
                  attributes = Cow::Owned(generate_attributes_string(&RefCell::new(vec![])));
                 },

          NodeData::Text { ref contents }
              => {
                    tag_name = Cow::Owned("text".to_string());
                    attributes = Cow::Owned(generate_attributes_string(&RefCell::new(vec![])));
                   },

          NodeData::Comment { ref contents }
              => {
                    tag_name = Cow::Owned("comment".to_string());
                    attributes = Cow::Owned(generate_attributes_string(&RefCell::new(vec![])));
                    },

          NodeData::Element { ref name, ref attrs, .. } => {
              tag_name = Cow::Owned(name.local.to_string());
              attributes = Cow::Owned(generate_attributes_string(attrs));
          }

          NodeData::ProcessingInstruction { .. } => unreachable!()
      }

      let children: String = generate_child_elements_string(&node.children);

      //let attributes = "vec![]";
      format!("h(\"{}\", {}, {})", tag_name.into_owned(), children, attributes.into_owned())
  }

  pub fn parse<'a>(html: String) -> String {
    let str = Tendril::from_str(&*html).unwrap();
    let mut parser = parse_document(RcDom::default(), Default::default());
    parser.process(str);
    let dom = parser.finish();

    return generate_create_element_string(0, dom.document);

  }
}