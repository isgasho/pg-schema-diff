use crate::schema_set::{Diff, Sql, SqlList};
use postgres_parser::nodes::DoStmt;
use postgres_parser::Node;

impl Sql for DoStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("DO ");
        sql.push_str(&self.args.sql(" "));

        sql
    }
}

impl Diff for DoStmt {}
