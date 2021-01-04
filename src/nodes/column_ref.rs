use crate::schema_set::SqlIdent;
use postgres_parser::nodes::ColumnRef;

impl SqlIdent for ColumnRef {
    fn sql_ident(&self) -> String {
        self.fields.sql_ident()
    }

    fn sql_ident_prefix(&self, pre: &str) -> String {
        format!("{}{}", pre, self.sql_ident())
    }

    fn sql_ident_suffix(&self, suf: &str) -> String {
        format!("{}{}", self.sql_ident(), suf)
    }
}
