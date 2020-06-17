#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    // fn test() {
    //     let result = (706, &file);
    // }
    #[test]
    fn test() {
        let file = include_str!("test_data/mdfile.xml").to_string();

        let doc = parse_rox(include_str!("reference/scriptproperties.xml"));
        let sp = ScriptProperties::parse(&doc);

        let typeref = type_checker::TypeRef {
            datatypes: sp.datatypes,
            keywords: sp.keywords,
            methods: ron_parse::parse_method_ron(include_str!("reference/methods.ron")),
            events: ron_parse::parse_event_ron(include_str!("reference/events.ron")),
        };
        println!("{:?}", typeref);
    }
}
