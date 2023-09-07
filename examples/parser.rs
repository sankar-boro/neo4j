use neo4j::parser::CypherParser;
use neo4j::parser::{parse, print_pairs};

fn main() {
    let code = "MATCH (n) WHERE n.name CONTAINS \"s\" RETURN n.name;";
    // let code = "MATCH (n) WHERE n.name CONTAINS12 RETURN n.name;";
    match parse(code) {
        Ok(tree) => print_pairs(tree),
        Err(err) => eprintln!("ERROR={}", err),
    }
        
    // use open_cypher::parser::parse_string_literal;
    // let text = "n.name";
    // match parse_string_literal(text) {
    //     Ok(tree) => print_pairs(tree),
    //     Err(err) => eprintln!("ERROR={}", err),
    // }
}