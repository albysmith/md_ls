mod graph;
use graph::parse_rox;
mod expression_parser;
mod ron_parse;
mod script_properties;
use script_properties::{ScriptProperties};
mod tests;
mod type_checker;
mod completion;
mod hover;

fn main() {
    println!("Hello, world!");
}

fn main_loop() {
    let doc = parse_rox(include_str!("reference/scriptproperties.xml"));
    let sp = ScriptProperties::parse(&doc);

    let typeref = type_checker::TypeRef {
        datatypes: sp.datatypes,
        keywords: sp.keywords,
        methods: ron_parse::parse_method_ron(include_str!("reference/methods.ron")),
        events: ron_parse::parse_event_ron(include_str!("reference/events.ron")),
    };
}
