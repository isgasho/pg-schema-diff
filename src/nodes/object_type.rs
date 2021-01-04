use crate::schema_set::Sql;
use postgres_parser::sys::ObjectType;

impl Sql for ObjectType {
    fn sql(&self) -> String {
        match self {
            ObjectType::OBJECT_ACCESS_METHOD => "ACCESS METHOD",
            ObjectType::OBJECT_AGGREGATE => "AGGREGATE",
            ObjectType::OBJECT_AMOP => "AMOP",
            ObjectType::OBJECT_AMPROC => "AMPROC",
            ObjectType::OBJECT_ATTRIBUTE => "ATTRIBUTE",
            ObjectType::OBJECT_CAST => "CAST",
            ObjectType::OBJECT_COLUMN => "COLUMN",
            ObjectType::OBJECT_COLLATION => "COLLATION",
            ObjectType::OBJECT_CONVERSION => "CONVERSION",
            ObjectType::OBJECT_DATABASE => "DATABASE",
            ObjectType::OBJECT_DEFAULT => "DEFAULT",
            ObjectType::OBJECT_DEFACL => "DEFACL",
            ObjectType::OBJECT_DOMAIN => "DOMAIN",
            ObjectType::OBJECT_DOMCONSTRAINT => "DOMCONSTRAINT",
            ObjectType::OBJECT_EVENT_TRIGGER => "EVENT TRIGGER",
            ObjectType::OBJECT_EXTENSION => "EXTENSION",
            ObjectType::OBJECT_FDW => "FDW",
            ObjectType::OBJECT_FOREIGN_SERVER => "FOREIGN SERVER",
            ObjectType::OBJECT_FOREIGN_TABLE => "FOREIGN TABLE",
            ObjectType::OBJECT_FUNCTION => "FUNCTION",
            ObjectType::OBJECT_INDEX => "INDEX",
            ObjectType::OBJECT_LANGUAGE => "LANGUAGE",
            ObjectType::OBJECT_LARGEOBJECT => "LARGEOBJECT",
            ObjectType::OBJECT_MATVIEW => "MATVIEW",
            ObjectType::OBJECT_OPCLASS => "OPCLASS",
            ObjectType::OBJECT_OPERATOR => "OPERATOR",
            ObjectType::OBJECT_OPFAMILY => "OPFAMILY",
            ObjectType::OBJECT_POLICY => "POLICY",
            ObjectType::OBJECT_PROCEDURE => "PROCEDURE",
            ObjectType::OBJECT_PUBLICATION => "PUBLICATION",
            ObjectType::OBJECT_PUBLICATION_REL => "PUBLICATION REL",
            ObjectType::OBJECT_ROLE => "ROLE",
            ObjectType::OBJECT_ROUTINE => "ROUTINE",
            ObjectType::OBJECT_RULE => "RULE",
            ObjectType::OBJECT_SCHEMA => "SCHEMA",
            ObjectType::OBJECT_SEQUENCE => "SEQUENCE",
            ObjectType::OBJECT_SUBSCRIPTION => "SUBSCRIPTION",
            ObjectType::OBJECT_STATISTIC_EXT => "STATISTIC EXT",
            ObjectType::OBJECT_TABCONSTRAINT => "TABCONSTRAINT",
            ObjectType::OBJECT_TABLE => "TABLE",
            ObjectType::OBJECT_TABLESPACE => "TABLESPACE",
            ObjectType::OBJECT_TRANSFORM => "TRANSFORM",
            ObjectType::OBJECT_TRIGGER => "TRIGGER",
            ObjectType::OBJECT_TSCONFIGURATION => "TSCONFIGURATION",
            ObjectType::OBJECT_TSDICTIONARY => "TSDICTIONARY",
            ObjectType::OBJECT_TSPARSER => "TSPARSER",
            ObjectType::OBJECT_TSTEMPLATE => "TSTEMPLATE",
            ObjectType::OBJECT_TYPE => "TYPE",
            ObjectType::OBJECT_USER_MAPPING => "USER MAPPING",
            ObjectType::OBJECT_VIEW => "VIEW",
        }
        .into()
    }
}