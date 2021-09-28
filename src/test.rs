#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use roxmltree::Document;

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
    #[test]
    fn should_send_new_message(){
        let photo_menu = get_valid_document(read_to_string("templates/test/photo_view.xml").unwrap().as_str());
        let message_menu = get_valid_document(read_to_string("templates/test/message_view.xml").unwrap().as_str());
        photo_menu.
    }
}