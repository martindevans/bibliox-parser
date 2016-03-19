//Core components
pub struct TableName {
    pub name: String,
    pub schema_name: Option<String>
}

pub struct ColumnDefinition {
    pub name: String,
    // todo: type
    // todo: constraints
}

//https://www.sqlite.org/syntax/sql-stmt.html
pub enum Statement {
    Plain(StatementBody),
    Explain(StatementBody),
    ExplainQueryPlan(StatementBody)
}

pub enum StatementBody {

    //https://www.sqlite.org/syntax/alter-table-stmt.html
    AlterTableRenameTo(TableName, TableName),
    AlterTableAddColumn(TableName, ColumnDefinition),

    /* Analyze(i32),
    Attach(i32),
    Begin(i32),
    Commit(i32),
    CreateIndex(i32),
    CreateTable(i32),
    CreateTrigger(i32),
    CreateView(i32),
    CreateVirtualTable(i32),
    Delete(i32),
    DeleteLimited(i32),
    Detach(i32),
    DropIndex(i32),
    DropTable(i32),
    DropTrigger(i32),
    DropView(i32),
    Insert(i32),
    Pragma(i32),
    Reindex(i32),
    Release(i32),
    Rollback(i32),
    Savepoint(i32),
    Select(i32),
    Update(i32),
    UpdateLimited(i32),
    Vacuum(i32), */
}
