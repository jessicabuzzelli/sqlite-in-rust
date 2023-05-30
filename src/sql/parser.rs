use sqlparser::ast::Statement;
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::{Parser, ParserError};

pub fn parse_sql_command(input: &String) -> Result<Vec<Statement>, ParserError> {
    let dialect = AnsiDialect{};
    let ast: Result<Vec<Statement>, ParserError> = Parser::parse_sql(&dialect, input);

    return ast
}

