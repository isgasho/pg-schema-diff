use crate::schema_set::{Diff, Sql, SqlIdent};
use postgres_parser::nodes::ClusterStmt;
use postgres_parser::sys::ClusterOption;

impl Sql for ClusterStmt {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("CLUSTER ");
        if self.options == ClusterOption::CLUOPT_VERBOSE as i32 {
            sql.push_str("VERBOSE ");
        }
        sql.push_str(&self.relation.sql());
        sql.push_str(&self.indexname.sql_ident_prefix(" USING "));

        sql
    }
}

impl Diff for ClusterStmt {}
