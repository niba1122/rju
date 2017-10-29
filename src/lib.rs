#![crate_type="proc-macro"]
#![feature(proc_macro)]
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let text = input.to_string();
    //print!("{}", html_parser::parse(text));
    //"h()".parse().unwrap()
    html_parser::parse(text).parse().unwrap()
}

  extern crate html5ever;

mod html_parser {
  use html5ever::parse_document;
  use html5ever::rcdom::{NodeData, RcDom, Handle};
  use html5ever::tendril::TendrilSink;
  use html5ever::tendril::Tendril;
  use html5ever::driver::Parser;
  use std::str::FromStr;
  use std::iter::repeat;
  use std::fmt;
  use std::fmt::Write;
  use std::cell::{RefCell};
  use std::borrow::Cow;

  fn generate_child_elements_string(children: &RefCell<Vec<Handle>>) -> String {
      let mut output: String = String::new();
      write!(&mut output, "vec![");
      for (i, child) in children.borrow().iter().enumerate() {
          if i > 0 {
              write!(&mut output, ",");
          }
          let child_str = generate_create_element_string(0, child.clone());
          write!(&mut output, "{}", child_str);
      }
      write!(&mut output, "]");
      format!("{}", output)
  }

  fn generate_create_element_string<'a>(indent: usize, handle: Handle) -> String {
      let output = String::new();

      let node = handle;
      let tag_name: Cow<'a, str>;
      match node.data {
          NodeData::Document
              => {
                  tag_name = Cow::Owned("document".to_string())
                },

          NodeData::Doctype { ref name, ref public_id, ref system_id }
              => {
                    tag_name = Cow::Owned("doctype".to_string())
                 },

          NodeData::Text { ref contents }
              => {
                    tag_name = Cow::Owned("text".to_string())
                   
                   },

          NodeData::Comment { ref contents }
              => {
                    tag_name = Cow::Owned("comment".to_string())
                    },

          NodeData::Element { ref name, ref attrs, .. } => {
              tag_name = Cow::Owned(name.local.to_string())
          }

          NodeData::ProcessingInstruction { .. } => unreachable!()
      }

      let children: String = generate_child_elements_string(&node.children);

      let attributes = "vec![]";
      format!("h(\"{}\", {}, {})", tag_name.into_owned(), children, attributes)
  }

  pub fn parse<'a>(html: String) -> String {
    let str = Tendril::from_str(&*html).unwrap();
    let mut parser = parse_document(RcDom::default(), Default::default());
    parser.process(str);
    let dom = parser.finish();
    return generate_create_element_string(0, dom.document);

  }
}