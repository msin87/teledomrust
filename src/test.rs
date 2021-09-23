#[cfg(test)]
mod tests {
    use rustdom::{AirSmellData, SmellStrength};
    use sailfish::TemplateOnce;
    use std::collections::HashMap;

    #[derive(TemplateOnce)]
    #[template(path="editdatamenu.stpl")]
    struct EditDataMenuTemplate<'a>{
        userId: i64,
        sqlIntResult: HashMap<&'a str, i32>
    }
    #[test]
    fn it_works(){
        // let air_smell = AirSmellData{
        //     id: 0,
        //     user_id: 0,
        //     smell_strength: SmellStrength::None,
        //     latitude: 0.0,
        //     longitude: 0.0,
        //     time_epoch: 0,
        //     comment: "",
        //     canceled: false,
        //     claims: vec![]
        // };
        let mut sqlIntResult = HashMap::new();
        sqlIntResult.insert(String::from("airSmellCount"),99);
        sqlIntResult.insert(String::from("otherCounter"),12);

        let ctx = EditDataMenuTemplate{
            userId: 545,
            sqlIntResult: Default::default()
        };
        println!("{}",ctx.render_once().unwrap());
        assert_eq!(2+2,4)
    }
}