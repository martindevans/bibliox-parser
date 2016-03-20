pub mod sql_parser;
pub mod ast;

#[cfg(test)]
mod tests {
    use std::fmt::{ Debug };
    use std::cmp::{ Eq };

    use super::sql_parser as sql_parser;
    use super::ast::{
        Statement,
        StatementBody,
        TableName,
        ColumnDefinition,
        Type,
        PrimaryKeyConstraint,
        ColumnConstraintType,
        ColumnConstraint,
        ConflictClause,
        CollateFunction
    };

    //Helpers
    fn parse_to_ast<'a, P, R, E>(input: &'a str, parser: P, expected: R)
        where P : Fn(&'a str) -> Result<R, E>,
              E : Debug,
              R : Debug, 'a,
              R : Eq
    {
        match parser(input) {
            Ok(r) => {
                if r != expected {
                    assert!(false, "Parsing \"{}\" got \n{:#?} expected \n{:#?}", input, r, expected)
                }
            }
            Err(err) => assert!(false, "{:#?}", err)
        }
    }

    fn parse_to_fail<'a, P, R, E>(input: &'a str, parser: P)
        where P : Fn(&'a str) -> Result<R, E>,
              E : Debug,
              R : Debug, 'a,
              R : Eq
    {
        match parser(input) {
            Ok(r) => {
                assert!(false, "Expected failure, succeeded with {:#?}", r)
            }
            Err(_) => { /* Expected failure :) */ }
        }
    }

    fn sql_statement_to_ast<'a>(sql: &'a str, expected: Statement<'a>) {
        parse_to_ast::<'a>(sql, sql_parser::parse_Statement, expected);
    }

    #[test]
    fn parse_table_name_no_schema() {
        parse_to_ast("tableName", sql_parser::parse_TableName, TableName {
            name: "tableName",
            schema_name: None
        });
    }

    #[test]
    fn parse_table_name_with_invalid_name() {
        parse_to_fail("#", sql_parser::parse_TableName);
    }

    #[test]
    fn parse_table_name_with_schema() {
        parse_to_ast("schema.tableName", sql_parser::parse_TableName, TableName {
            name: "tableName",
            schema_name: Some("schema")
        });
    }

    #[test]
    fn parse_table_name_with_invalid_schema() {
        parse_to_fail("#.tableName", sql_parser::parse_TableName);
    }

    #[test]
    fn parse_type_with_name() {
        parse_to_ast("INT", sql_parser::parse_Type, Type {
            name: "INT",
            num1: None,
            num2: None,
        });
    }

    #[test]
    fn parse_type_with_name_and_num() {
        parse_to_ast("INT(1)", sql_parser::parse_Type, Type {
            name: "INT",
            num1: Some(1),
            num2: None,
        });
    }

    #[test]
    fn parse_type_with_name_and_nums() {
        parse_to_ast("INT(1,2)", sql_parser::parse_Type, Type {
            name: "INT",
            num1: Some(1),
            num2: Some(2),
        });
    }

    #[test]
    fn alter_table_rename() {
        sql_statement_to_ast("ALTER TABLE schema.table RENAME TO foo",
            Statement::Plain(
                StatementBody::AlterTableRenameTo(
                    TableName {
                        schema_name: Some("schema"),
                        name: "table"
                    },
                    TableName {
                        schema_name: None,
                        name: "foo"
                    },
                )
            )
        );
    }

    #[test]
    fn alter_table_add_column() {
        sql_statement_to_ast("ALTER TABLE table ADD COLUMN column",
            Statement::Plain(
                StatementBody::AlterTableAddColumn(
                    TableName {
                        schema_name: None,
                        name: "table"
                    },
                    ColumnDefinition {
                        name: "column",
                        type_def: None,
                        constraints: Vec::new()
                    }
                )
            )
        );
    }

    #[test]
    fn explain_alter_table_add_column() {
        sql_statement_to_ast("EXPLAIN ALTER TABLE table ADD COLUMN column",
            Statement::Explain(
                StatementBody::AlterTableAddColumn(
                    TableName {
                        schema_name: None,
                        name: "table"
                    },
                    ColumnDefinition {
                        name: "column",
                        type_def: None,
                        constraints: Vec::new()
                    }
                )
            )
        );
    }

    #[test]
    fn parse_explain_query_plan_wraps_correct_ast() {
        sql_statement_to_ast("EXPLAIN QUERY PLAN ALTER TABLE table ADD COLUMN column",
            Statement::ExplainQueryPlan(
                StatementBody::AlterTableAddColumn(
                    TableName {
                        schema_name: None,
                        name: "table"
                    },
                    ColumnDefinition {
                        name: "column",
                        type_def: None,
                        constraints: Vec::new()
                    }
                )
            )
        );
    }

    #[test]
    fn parse_alter_with_constraint() {
        sql_statement_to_ast("EXPLAIN ALTER TABLE table ADD COLUMN column INT(1, 2) PRIMARY KEY ASC ON CONFLICT ABORT AUTOINCREMENT",
            Statement::Explain(
                StatementBody::AlterTableAddColumn(
                    TableName {
                        schema_name: None,
                        name: "table"
                    },
                    ColumnDefinition {
                        name: "column",
                        type_def: Some(Type {
                            name: "INT",
                            num1: Some(1),
                            num2: Some(2)
                        }),
                        constraints: vec![
                            ColumnConstraint {
                                name: None,
                                constraint: ColumnConstraintType::PrimaryKey(PrimaryKeyConstraint {
                                    ascending: Some(true),
                                    conflict: Some(ConflictClause::Abort),
                                    auto_increment: true
                                })
                            }
                        ]
                    }
                )
            )
        );
    }

    #[test]
    fn parse_column_constraint_primary_key() {
        parse_to_ast("PRIMARY KEY", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::PrimaryKey(PrimaryKeyConstraint {
                    ascending: None,
                    conflict: None,
                    auto_increment: false
                })
            }
        });
    }

    #[test]
    fn parse_column_constraint_primary_key_with_sort_order() {
        parse_to_ast("PRIMARY KEY DESC", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::PrimaryKey(PrimaryKeyConstraint {
                    ascending: Some(false),
                    conflict: None,
                    auto_increment: false
                })
            }
        });
    }

    #[test]
    fn parse_column_constraint_primary_key_with_conflict() {
        parse_to_ast("PRIMARY KEY ON CONFLICT ROLLBACK", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::PrimaryKey(PrimaryKeyConstraint {
                    ascending: None,
                    conflict: Some(ConflictClause::Rollback),
                    auto_increment: false
                })
            }
        });
    }

    #[test]
    fn parse_column_constraint_primary_key_with_autoincrement() {
        parse_to_ast("PRIMARY KEY AUTOINCREMENT", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::PrimaryKey(PrimaryKeyConstraint {
                    ascending: None,
                    conflict: None,
                    auto_increment: true
                })
            }
        });
    }

    #[test]
    fn parse_column_constraint_not_null() {
        parse_to_ast("NOT NULL", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::NotNull(None)
            }
        });
    }

    #[test]
    fn parse_column_constraint_not_null_with_conflict() {
        parse_to_ast("NOT NULL ON CONFLICT FAIL", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::NotNull(Some(ConflictClause::Fail))
            }
        });
    }

    #[test]
    fn parse_column_constraint_unique() {
        parse_to_ast("UNIQUE", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::Unique(None)
            }
        });
    }

    #[test]
    fn parse_column_constraint_unique_with_conflict() {
        parse_to_ast("UNIQUE ON CONFLICT REPLACE", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::Unique(Some(ConflictClause::Replace))
            }
        });
    }

    #[test]
    fn parse_column_constraint_collate() {
        parse_to_ast("COLLATE RTRIM", sql_parser::parse_ColumnConstraint, {
            ColumnConstraint {
                name: None,
                constraint: ColumnConstraintType::Collate(CollateFunction::RightTrim)
            }
        });
    }
}
