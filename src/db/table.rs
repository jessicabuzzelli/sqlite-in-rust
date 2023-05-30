use std::iter::Map;

struct Database {
    pub name: string,
    pub tables: Map<string, Table>,
}

struct Table {
    pub name: string,
    pub columns: Map<string, Column>,
}

enum ColumnDataType {
    Boolean,
    String,
    Integer,
    Float,
}

struct Column {
    pub name: string,
    pub datatype: ColumnDataType,
}

impl Column {
    pub fn new(name: string, datatype: ColumnDataType) -> Column {
        return Column{name, datatype}
    }
}

impl Table {
    pub fn new(name: string) -> Table {
        return Table{name, columns: map!{} }
    }

    pub fn add_column(&mut self, column: Column) -> &self {
        self.columns.push(column);

        return self
    }

    pub fn drop_column(&mut self, column: Column) -> &self {
        self.columns.remove(column);

        return self
    }
}

impl Database {
    pub fn new(name: string) -> Database {
        return Database{name, tables: map!{}}
    }

    pub fn create_table(&mut self, table: Table) -> &self {
        self.tables.push(table);

        return self
    }
}
