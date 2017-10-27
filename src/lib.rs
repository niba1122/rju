#![crate_type="proc-macro"]
#![feature(proc_macro)]
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let text = input.to_string();
    html_parser::parse(text);
    "h()".parse().unwrap()
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

  fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
  }

  fn walk(indent: usize, handle: Handle) {
      let node = handle;
      // FIXME: don't allocate
      match node.data {
          NodeData::Document
              //=> println!("#Document"),
              => print!(""),

          NodeData::Doctype { ref name, ref public_id, ref system_id }
              => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

          NodeData::Text { ref contents }
              //=> println!("#text: {}", escape_default(&contents.borrow())),
              => print!(""),

          NodeData::Comment { ref contents }
              //=> println!("<!-- {} -->", escape_default(contents)),
              => println!(""),

          NodeData::Element { ref name, ref attrs, .. } => {
              /*print!("<{}", name.local);
              for attr in attrs.borrow().iter() {
                  print!(" {}=\"{}\"", attr.name.local, attr.value);
              }
              println!(">");*/
              print!("{}", repeat(" ").take(indent).collect::<String>());
              println!("h(\"{}\", [\n", name.local);
          }

          NodeData::ProcessingInstruction { .. } => unreachable!()
      }

      for child in node.children.borrow().iter() {
          walk(indent+4, child.clone());
      }
        let hoge = node.children.borrow().iter().map(|x| {
        String::from("hoge")
        }).collect::<Vec<String>>().join(",");
        println!("{:?}", hoge);

      match node.data {
          NodeData::Element{ ref name, ref attrs, .. }
            => {
              print!("{}", repeat(" ").take(indent).collect::<String>());
              println!("])");
            },
            _ => {}
      }
  }

  pub fn parse(html: String) {
    let str = Tendril::from_str(&*html).unwrap();
    let mut parser = parse_document(RcDom::default(), Default::default());
    parser.process(str);
    let dom = parser.finish();
    walk(0, dom.document);
    //println!("{:?}", dom.document.children.borrow().iter().aldskfj());

  }
}
/*
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("rn", expand_rn);
    reg.register_macro("html", expand_html);
}
*/