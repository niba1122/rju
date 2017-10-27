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

  fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
  }

  fn walk(indent: usize, handle: Handle) -> String {
      let mut output = String::new();

      let node = handle;
      write!(&mut output, "{}", repeat(" ").take(indent).collect::<String>());
      print!("{}", repeat(" ").take(indent).collect::<String>());
      match node.data {
          NodeData::Document
              //=> print!("h(\"document\", "),
              => {
                  write!(&mut output, "h(\"document\", ");
                    // print!("h(\"document\", ");
               },

          NodeData::Doctype { ref name, ref public_id, ref system_id }
              //=> println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),
              => {
                   write!(&mut output, "h(\"doctype\", ");
                //    print!("h(\"doctype\", ");
                 },

          NodeData::Text { ref contents }
              //=> println!("#text: {}", escape_default(&contents.borrow())),
              => {
                   write!(&mut output, "h(\"text\", ");
                //    print!("h(\"text\", ");
                   
                   },

          NodeData::Comment { ref contents }
              //=> println!("<!-- {} -->", escape_default(contents)),
              => {
                   write!(&mut output, "h(\"comment\", ");
                //    print!("h(\"comment\", ");
                    },

          NodeData::Element { ref name, ref attrs, .. } => {
              /*print!("<{}", name.local);
              for attr in attrs.borrow().iter() {
                  print!(" {}=\"{}\"", attr.name.local, attr.value);
              }
              println!(">");*/
              write!(&mut output, "h(\"{}\", ", name.local);
            //   print!("h(\"{}\", ", name.local);
          }

          NodeData::ProcessingInstruction { .. } => unreachable!()
      }

    //   print!("[\n");
      //write!(&mut output, "[\n");
      write!(&mut output, "vec![\n");
      for (i, child) in node.children.borrow().iter().enumerate() {
          if i > 0 {
              write!(&mut output, ",\n");
            //   print!(",\n");
          }
          let child_str = walk(indent+4, child.clone());
          write!(&mut output, "{}", child_str);
      }
    //   print!("\n");
    //   print!("{}", repeat(" ").take(indent).collect::<String>());
    //   print!("])");
      write!(&mut output, "\n");
      write!(&mut output, "{}", repeat(" ").take(indent).collect::<String>());
      write!(&mut output, "])");

      return output;
  }

  pub fn parse(html: String) -> String {
    let str = Tendril::from_str(&*html).unwrap();
    let mut parser = parse_document(RcDom::default(), Default::default());
    parser.process(str);
    let dom = parser.finish();
    return walk(0, dom.document);
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