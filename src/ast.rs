//Core components
#[derive(PartialEq, Eq, Debug)]
pub struct TableName<'a> {
    pub name: &'a str,
    pub schema_name: Option<&'a str>
}

#[derive(PartialEq, Eq, Debug)]
pub struct ColumnConstraint<'a> {
    pub name: Option<&'a str>,
    pub constraint: ColumnConstraintType
}

#[derive(PartialEq, Eq, Debug)]
pub struct PrimaryKeyConstraint {
    pub ascending: Option<bool>,
    pub conflict: Option<ConflictClause>,
    pub auto_increment: bool
}

//https://www.sqlite.org/syntax/column-constraint.html
#[derive(PartialEq, Eq, Debug)]
pub enum ColumnConstraintType {
    PrimaryKey(PrimaryKeyConstraint),
    NotNull(Option<ConflictClause>),
    Unique(Option<ConflictClause>),
    //Check,
    //Default,
    Collate(CollateFunction),
    //Foreign
}

#[derive(PartialEq, Eq, Debug)]
pub enum ConflictClause {
    Rollback,
    Abort,
    Fail,
    Ignore,
    Replace
}

//https://www.sqlite.org/datatype3.html#collation
#[derive(PartialEq, Eq, Debug)]
pub enum CollateFunction {
    Binary,
    NoCase,
    RightTrim
}

#[derive(PartialEq, Eq, Debug)]
pub struct ColumnDefinition<'a> {
    pub name: &'a str,
    pub type_def: Option<Type<'a>>,
    pub constraints: Vec<ColumnConstraint<'a>>
}

#[derive(PartialEq, Eq, Debug)]
pub struct Type<'a> {
    pub name: &'a str,
    pub num1: Option<i32>,
    pub num2: Option<i32>
}

//https://www.sqlite.org/syntax/sql-stmt.html
#[derive(PartialEq, Eq, Debug)]
pub enum Statement<'a> {
    Plain(StatementBody<'a>),
    Explain(StatementBody<'a>),
    ExplainQueryPlan(StatementBody<'a>)
}

#[derive(PartialEq, Eq, Debug)]
pub enum StatementBody<'a> {

    //https://www.sqlite.org/syntax/alter-table-stmt.html
    AlterTableRenameTo(TableName<'a>, TableName<'a>),
    AlterTableAddColumn(TableName<'a>, ColumnDefinition<'a>),

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
