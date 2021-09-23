#![feature(proc_macro_hygiene, decl_macro)]
use mysql::*;
use mysql::prelude::*;
use std::collections::HashMap;
use std::fs;
use sailfish::TemplateOnce;
use std::path::Path;
use roxmltree::Document;

#[derive(TemplateOnce)]
#[template(path="editdatamenu.stpl")]
struct EditDataMenuTemplate{
    userId: i64,
    sqlIntResult: HashMap<String, i32>,
    UPDATE: UPDATE
}
struct UPDATE{
    message: Message
}
struct Message{
    from: User
}
struct User{
    id: i64
}
fn main() {
    let mut sqlIntResult = HashMap::new();
    let url = "mysql://msin87yand:Solovagem2@VH249.spaceweb.ru:3306/msin87yand";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let path = Path::new("templates/editdatamenu.stpl");
    let txt = fs::read_to_string(path).expect("Can't read template");
    println!("{}",txt);
    let doc = match roxmltree::Document::parse(txt.as_str()) {
        Ok(doc) => doc,
        Err(e) => {
            println!("{}",e);
            panic!("Can't parse xml {:?}", e)
        }
    };
    let result = conn.query_fold("SELECT COUNT(*) FROM airdata", 0,|acc,row:i32|{
        acc+row
    }).unwrap();
    sqlIntResult.insert(String::from("airSmellCount"),0);
    sqlIntResult.insert(String::from("otherCounter"),12);
    sqlIntResult.insert(String::from("airSmokeCount"),16);

    let ctx = EditDataMenuTemplate{
        userId: 545,
        sqlIntResult,
        UPDATE: UPDATE {
            message: Message { from: User { id: 33 } }
        }
    };
    println!("{:?}",result);

}