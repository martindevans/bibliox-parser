pub mod sql_parser;
pub mod sql_syntax_tree;

#[cfg(test)]
fn test_it(sql: &'static str) {
    match sql_parser::parse_Statement(sql) {
        Ok(_) => {},
        Err(err) => {
            assert!(false, "{:#?}", err);
        }
    }
}

#[test]
fn it_works() {

    test_it("ALTER TABLE schema.table RENAME TO foo");
    test_it("ALTER TABLE table ADD COLUMN foo");
    test_it("EXPLAIN ALTER TABLE table ADD COLUMN foo");
    test_it("EXPLAIN QUERY PLAN ALTER TABLE table ADD COLUMN foo");

}
