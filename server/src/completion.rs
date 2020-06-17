use super::graph::{build_graph, get_byte_pos_in_string, parse_rox};
use lsp_types::{CompletionItem, CompletionParams};

pub fn handle_completion(
    params: CompletionParams,
    string: &String,
    typeref: &super::type_checker::TypeRef,
) -> Vec<CompletionItem> {
    let byte_pos = get_byte_pos_in_string(
        string,
        params.text_document_position.position.line as usize,
        params.text_document_position.position.character as usize,
    );
    let doc = parse_rox(string);

    let var_hash = build_graph(byte_pos, &doc, typeref);
    // USE GRAPH TO GET COMPLETION ITEMS
    // Need variables in scope + available script properties + available event params??
    vec![]
}
