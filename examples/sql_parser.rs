use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {
    
    let dialect = GenericDialect {}; // or AnsiDialect
    
    let sql = "MATCH (n) WHERE n.name = 'Alice' RETURN n";
    
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    
    println!("AST: {:?}", ast);
}