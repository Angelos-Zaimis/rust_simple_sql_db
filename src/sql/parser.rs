use crate::database::Database;
use sqlparser::ast::Statement;

pub fn execute_sql(statements: Vec<Statement>, db: &mut Database) {
    for statement in statements {
        match statement {
            Statement::Query(query) => {
                
            }
            Statement::Insert{} => {

            }
            Statement::Update{} => {

            }
            Statement::Delete{} => {

            }
            _ => {
                
            }
        }
    }
}