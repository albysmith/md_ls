#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test() {
        let file = include_str!("test_data/mdfile.xml").to_string();
        let result = completion::get_completion_items(706, &file);
        
    }
    #[test]
    fn scriptproperties() {
        let string = include_str!("reference/scriptproperties.xml");
        if let Ok(doc) = roxmltree::Document::parse(string) {
            let sp = script_properties::ScriptProperties::parse(&doc);
            println!("{:#?}", sp)
        }
    }
}

