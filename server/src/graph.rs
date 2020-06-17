use crate::expression_parser;
use crate::type_checker::{DataTypes, TypeRef};
use lsp_types::Url;
use roxmltree::*;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct Namespace<'a, 'input> {
    pub variables: HashMap<&'a str, Variable<'a>>,
    pub rox: Node<'a, 'input>,
}

#[derive(Debug, Default, Clone)]
pub struct Variable<'a> {
    pub name: &'a str,
    pub datatype: Vec<DataTypes>,
    pub origin: Position,
    pub references: Vec<Position>,
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

pub fn build_graph<'a>(
    byte_pos: usize,
    doc: &'a Document,
    typeref: &TypeRef,
) -> Option<HashMap<&'a str, Variable<'a>>> {
    // find where we are in the file
    // find all available variables
    // find all available scriptproperties
    // find all available event params

    for mdscript in doc.root().children() {
        if mdscript.tag_name().name() == "mdscript" {
            // println!("mdscript match");
            let mut namespace = Namespace {
                variables: HashMap::new(),
                rox: mdscript,
            };

            for child in mdscript.children() {
                match child.tag_name().name() {
                    "cues" => {
                        for cue in child.children() {
                            if cue.range().start < byte_pos && cue.range().end > byte_pos {
                                match cue.tag_name().name() {
                                    "cue" => {
                                        namespace.rox = cue;
                                        process_cue(&mut namespace, byte_pos, typeref)
                                    }
                                    "library" => {
                                        namespace.rox = cue;
                                        process_cue(&mut namespace, byte_pos, typeref)
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    "" => {}
                    _ => {}
                }
            }
            return Some(namespace.variables);
        }
    }
    None
}

pub fn open_file_from_uri(uri: &Url) -> Option<String> {
    if let Ok(path) = uri.to_file_path() {
        if let Ok(string) = fs::read_to_string(path) {
            return Some(string);
        }
    }
    None
}

pub fn parse_rox(string: &str) -> roxmltree::Document {
    let doc = match roxmltree::Document::parse(string) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load xml file: {}", e);
            std::process::exit(1);
        }
    };
    doc
}

pub fn get_byte_pos_in_string(string: &String, line: usize, character: usize) -> usize {
    let mut byte_pos = 0;
    for (i, line_str) in string.lines().enumerate() {
        if i == line {
            byte_pos += character;
            return byte_pos;
        } else {
            byte_pos += line_str.len() + 2;
        }
    }
    byte_pos
}

fn process_cue(namespace: &mut Namespace, byte_pos: usize, typeref: &TypeRef) {
    if namespace.rox.tag_name().name() == "library" {
        namespace.variables = HashMap::new();
    } else if let Some(this) = namespace.rox.attribute("namespace") {
        if this == "this" {
            namespace.variables = HashMap::new();
        }
    }
    for node in namespace.rox.children() {
        match node.tag_name().name() {
            "conditions" => {
                namespace.rox = node;
                process_nodes(namespace, typeref)
            }
            "delay" => {
                namespace.rox = node;
                process_nodes(namespace, typeref)
            }
            "actions" => {
                namespace.rox = node;
                process_nodes(namespace, typeref)
            }
            "patch" => {
                namespace.rox = node;
                process_nodes(namespace, typeref)
            }
            "cues" => {
                for cue in node.children() {
                    if cue.range().start < byte_pos && cue.range().end > byte_pos {
                        match cue.tag_name().name() {
                            "cue" => {
                                namespace.rox = cue;
                                process_cue(namespace, byte_pos, typeref)
                            }
                            "library" => {
                                namespace.rox = cue;
                                process_cue(namespace, byte_pos, typeref)
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn process_nodes(namespace: &mut Namespace, typeref: &TypeRef) {
    for node in namespace.rox.descendants() {
        match node.tag_name().name() {
            "" => {}
            " " => {}
            "actions" => {}
            "conditions" => {}
            "patch" => {}
            _ => {
                for n in node.descendants() {
                    for a in n.attributes() {
                        // now we have the attribute
                        // check if it has any variables in it; no point including if not

                        if a.value().contains("$") || a.value().contains(".") {
                            // parse expressions to get variables out
                            // check if it is created on this node and use that type
                            // need to look back at prior uses of this variable to get type if not immediately accessible
                            // need to do aliasing for set values
                            // need to include scriptproperties!!!!!
                            if let Some(results) = expression_parser::parse_expression(a.value()) {
                                let mut prior_value = &expression_parser::Parsed::default();
                                for parsed in results.iter() {
                                    match parsed.token {
                                        expression_parser::Token::Error => {}
                                        expression_parser::Token::Keyword(value) => {}
                                        expression_parser::Token::ScriptProperty(value) => {}
                                        expression_parser::Token::Variable(value) => {
                                            if let Some(var) = namespace.variables.get_mut(value) {
                                                var.references.push(Position {
                                                    start: parsed.pos.start + a.value_range().start,
                                                    end: parsed.pos.end + a.value_range().start,
                                                });
                                                // UPDATE TYPE INFO AS NEEDED
                                                // check to see if this node is a method that's creating it
                                                // let output: Vec<&super::ron_parse::Output> = typeref.methods.iter().filter_map(|m| {
                                                //     m.search_output(
                                                //         node.tag_name().name(),
                                                //         a.name(),
                                                //         node.attribute("multiple")
                                                //     )
                                                // }).collect();
                                                prior_value = parsed;
                                            } else {
                                                // GET TYPE INFO

                                                prior_value = parsed;
                                                let ok = namespace.variables.insert(
                                                    value,
                                                    Variable {
                                                        name: value,
                                                        datatype: vec![DataTypes::Unknown],
                                                        origin: Position {
                                                            start: parsed.pos.start
                                                                + a.value_range().start,
                                                            end: parsed.pos.end
                                                                + a.value_range().start,
                                                        },
                                                        ..Default::default()
                                                    },
                                                );
                                            }
                                        }
                                    }
                                }

                                // let ok = results
                                //     .into_iter()
                                //     .map(|t| match t.0 {
                                //         expression_parser::Token::Error => {}
                                //         expression_parser::Token::Variable(value)
                                //         | expression_parser::Token::ScriptProperty(value)
                                //         | expression_parser::Token::Keyword(value) => {
                                //             if let Some(var) = namespace.variables.get_mut(value) {
                                //                 var.references.push(Position {
                                //                     start: t.1.start + a.value_range().start,
                                //                     end: t.1.end + a.value_range().start,
                                //                 });
                                //             // UPDATE TYPE INFO AS NEEDED
                                //             } else {
                                //                 // GET TYPE INFO
                                //                 let ok = namespace.variables.insert(
                                //                     value,
                                //                     Variable {
                                //                         name: value,
                                //                         datatype: DataTypes::Unknown,
                                //                         origin: Position {
                                //                             start: t.1.start
                                //                                 + a.value_range().start,
                                //                             end: t.1.end + a.value_range().start,
                                //                         },
                                //                         ..Default::default()
                                //                     },
                                //                 );
                                //             }
                                //         }
                                //     })
                                //     .collect::<Vec<_>>();
                            }
                        }
                    }
                }
            }
        }
    }
}
