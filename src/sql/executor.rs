use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::Statement;


pub fn parse_sql(query: &str) -> Result<Vec<Statement>, sqlparser::parser::ParserError> {
    let dialect = GenericDialect {};
    Parser::parse_sql(&dialect. query)
}