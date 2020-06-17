use super::graph::{build_graph, get_byte_pos_in_string, parse_rox};
use lsp_types::{HoverContents, HoverParams};

// pub fn handle_completion(
//     params: HoverParams,
//     string: &String,
//     typeref: &super::type_checker::TypeRef,
// ) -> HoverContents {
//     let byte_pos = get_byte_pos_in_string(
//         string,
//         params.text_document_position_params.position.line as usize,
//         params.text_document_position_params.position.character as usize,
//     );
//     let doc = parse_rox(string);

//     let var_hash = build_graph(byte_pos, &doc, typeref);
//     // USE GRAPH TO GET HOVER ITEM
//     // Need variables in scope + available script properties + available event params??
    
// }
