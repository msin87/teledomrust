#![feature(proc_macro_hygiene, decl_macro)]

mod test;

use mysql::*;
use mysql::prelude::*;
use std::collections::HashMap;
use std::fs;
use sailfish::TemplateOnce;
use std::path::Path;
use roxmltree::{Document, Node};
use reqwest::RequestBuilder;
use crate::SendMethod::{Bypass, Edit, SendNew};

#[derive(TemplateOnce)]
#[template(path = "editdatamenu/editdatamenu.stpl")]
struct EditDataMenuTemplate<'a> {
    update: Update,
    sql_connection: &'a mut Conn,
}

struct Update {
    message: Message,
}

struct Message {
    from: User,
}

struct User {
    id: i64,
}

fn get_valid_document(text: &str) -> Option<Document> {
    match roxmltree::Document::parse(text) {
        Ok(document) => {
            if document.root_element().tag_name().name().eq("view") {
                return Some(document);
            }
            None
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

// fn is_text_tag_eq(first_node: Node, second_node: Node) -> bool {
//     let first_text = first_node.first_child().unwrap().next_siblings().find(|node| node.tag_name().name().eq("text"));
//     let second_text = second_node.first_child().unwrap().next_siblings().find(|node| node.tag_name().name().eq("text"));
//     if first_text.is_some() && second_text.is_some() {
//         first_text.unwrap().text().unwrap().eq(second_text.unwrap().text().unwrap())
//     }
//     false
// }

enum SendMethod {
    SendNew,
    Edit,
    Bypass,
}

// fn get_text_send_method(old_node: Option<Node>, new_node: Node) -> SendMethod {
//     //Если
//     if old_node.is_some() {
//         match is_text_tag_eq(old_node.unwrap(), new_node) {
//             true => text_send_method = Bypass,
//             false => text_send_method = Edit
//         }
//     }
//     SendNew
// }


//
// fn transform_message(old_node: Option<Node>, new_node: Node, bot_token: &str) -> Option<Result<RequestBuilder>> {
//     let text_send_method: SendMethod;
//     let keyboard_send_method: SendMethod;
//
//     match text_send_method {
//         SendNew => {}
//         Edit => {}
//         Bypass => {}
//     }
// }

fn main() {
    let url = "mysql://msin87yand:Solovagem2@VH249.spaceweb.ru:3306/msin87yand";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let ctx = EditDataMenuTemplate {
        update: Update {
            message: Message { from: User { id: 1 } }
        },
        sql_connection: conn.as_mut(),
    };
    let text: String = ctx.render_once().unwrap();
    println!("{}", text);
    // let doc_option = get_valid_document(text.as_str());
    // if doc_option.is_none() {
    //     return;
    // }
    // let doc = doc_option.unwrap();
    // let root = doc.root_element();
    // let child = root.first_child();
    // if child.is_some() {
    //     for sibling in child.unwrap().next_siblings() {
    //         println!("{}", sibling.tag_name().name());
    //     }
    // }
}