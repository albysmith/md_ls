
mod script_properties;
mod completion;
mod ron_parse;
mod tests;
mod expression_parser;
mod type_checker;

fn main() {
    println!("Hello, world!");
}

fn main_loop() {

    let mut sp = script_properties::ScriptProperties::default();
    if let Ok(doc) = roxmltree::Document::parse(include_str!("reference/scriptproperties.xml")) {
        sp = script_properties::ScriptProperties::parse(&doc);
    }

}