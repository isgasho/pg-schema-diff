use crate::schema_set::{Sql, SqlIdent, SqlList};
use postgres_parser::nodes::CommonTableExpr;

impl Sql for CommonTableExpr {
    fn sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str(&self.ctename.sql_ident());
        sql.push_str(&self.aliascolnames.sql_wrap(", ", "(", ")"));
        sql.push_str(" AS (");
        sql.push_str(&self.ctequery.sql());
        sql.push_str(") ");

        sql
    }
}
