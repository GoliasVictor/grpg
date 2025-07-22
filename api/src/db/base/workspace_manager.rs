use std::collections::HashMap;
use crate::db::models::TableDefinition;
use crate::db::base::{
    Store,
};
pub struct WorkspaceManager<'a> {
    pub store: &'a Store,
    pub workspace: i32,
}
use crate::db::base::tables::{
    set_table,
    get_table,
    get_tables,
    add_table,
    remove_table
};
impl WorkspaceManager<'_> {
    pub fn set_table(&self, id: i32, table: TableDefinition) -> Result<(), String> {
        set_table(self.store, self.workspace, id, table)
    }
    pub fn get_table(&self, id: i32) -> Option<TableDefinition> {
        get_table(self.store, self.workspace, id)
    }
    pub fn get_tables(&self) -> Option<HashMap<i32, TableDefinition>> {
        get_tables(self.store, self.workspace)
    }
    pub fn add_table(&self, table: TableDefinition) -> Option<i32> {
        add_table(self.store, self.workspace, table)
    }
    pub fn remove_table(&self, id: i32) -> Option<TableDefinition> {
        remove_table(self.store, self.workspace, id)
    }
}
