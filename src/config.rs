use std::io;

// use std::fs::File;
// use std::io::BufReader;
use std::fs;

use crate::xml_parser::*;

// use minidom::*;
extern crate minidom;
// use xml::reader::{EventReader, XmlEvent};

// fn indent(size: usize) -> String {
//     const INDENT: &'static str = "    ";
//     (0..size).map(|_| INDENT)
//              .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
// }

pub struct Config {
    cabs: Vec<KnownCab>,
    ip_addr: String,
    serial_port: String,
    legacy_mode: bool,
    debug: bool,
    logging: bool,
}

pub struct KnownCab {
    pub name: String,
    pub address: u32,
}
// const DATA: &'static str = r#"<articles xmlns="article">
//     <article>
//         <title>10 Terrible Bugs You Would NEVER Believe Happened</title>
//         <body>
//             Rust fixed them all. &lt;3
//         </body>
//     </article>
//     <article>
//         <title>BREAKING NEWS: Physical Bug Jumps Out Of Programmer's Screen</title>
//         <body>
//             Just kidding!
//         </body>
//     </article>
// </articles>"#;

// const ARTICLE_NS: &'static str = "article";

#[derive(Debug)]
pub struct Article {
    title: String,
    body: String,
}

impl Config {
    pub fn load(filename: String) -> std::result::Result<Config, io::Error> { // Need to remove underscore before filename
        let cabs: Vec<KnownCab> = vec!();

        // let file = File::open(filename).unwrap();
        // let file = BufReader::new(file);
        // let root: Element = DATA.parse().unwrap();

        // let mut articles: Vec<Article> = Vec::new();

        // for child in root.children() {
        //     if child.is("article", ARTICLE_NS) {
        //         let title = child.get_child("title", ARTICLE_NS).unwrap().text();
        //         let body = child.get_child("body", ARTICLE_NS).unwrap().text();
        //         articles.push(Article {
        //             title: title,
        //             body: body.trim().to_owned(),
        //         });
        //     }
        // }

        // println!("{:#?}", root);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        println!("{}", contents);
        let xml_nodes = XMLParser::parse(contents);
        println!("{:?}", xml_nodes);
        println!("{:?}", xml_nodes.get_value(vec!("config", "server", "ip")));
        println!("{:?}", xml_nodes.get_value(vec!("config", "cabs", "cab", "name")));
        // let root: minidom::Element = contents.parse().unwrap();
        // println!("{:#?}", root);

        // let parser = EventReader::new(file);
        // let mut depth = 0;
        // for e in parser {
        //     match e {
        //         Ok(XmlEvent::StartElement { name, .. }) => {
        //             println!("{}+{}", indent(depth), name);
        //             depth += 1;
        //         }
        //         Ok(XmlEvent::EndElement { name }) => {
        //             depth -= 1;
        //             println!("{}-{}", indent(depth), name);
        //         }
        //         Err(e) => {
        //             println!("Error: {}", e);
        //             break;
        //         }
        //         _ => {}
        //     }
        // }

        Ok(Config{
            cabs: cabs,
            ip_addr: String::new(),
            serial_port: String::new(),
            legacy_mode: false,
            debug: false,
            logging: false,
        })
    }
}
