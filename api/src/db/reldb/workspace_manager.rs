use std::collections::HashMap;
use crate::db::models::TableDefinition;
use crate::db::reldb::{
    Store,
};
pub struct WorkspaceManager<'a> {
    pub store: &'a Store,
    pub workspace: i32,
}
use crate::db::reldb::tables::{
    set_table,
    get_table,
    get_tables,
    add_table,
    remove_table
};
impl WorkspaceManager<'_> {
    pub async fn set_table(&self, id: i32, table: TableDefinition) -> Result<(), String> {
        set_table(self.store, self.workspace, id, table).await
    }
    pub async fn get_table(&self, id: i32) -> Option<TableDefinition> {
        get_table(self.store, self.workspace, id).await
    }
    pub async fn get_tables(&self) -> Option<HashMap<i32, TableDefinition>> {
        get_tables(self.store, self.workspace).await
    }
    pub async fn add_table(&self, table: TableDefinition) -> Option<i32> {
        add_table(self.store, self.workspace, table).await
    }
    pub async fn remove_table(&self, id: i32) -> Option<TableDefinition> {
        remove_table(self.store, self.workspace, id).await
    }
}
