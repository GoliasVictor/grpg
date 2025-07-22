use std::collections::HashMap;
use crate::db::models::TableDefinition;
use crate::db::base::{
    Store,
};
use crate::db::base::workspaces::{
    read_workspace,
    save_workspace
};

pub fn set_table(store: &Store, workspace_id: i32, id: i32, table: TableDefinition) -> Result<(), String> {
    let _lock = store.0.lock().unwrap();
    let mut store_data = read_workspace(store, workspace_id).ok_or("Workspace not found")?;
    store_data.tables.insert(id, table);
    save_workspace(store, workspace_id, store_data);
    Ok(())
}
pub fn get_table(store: &Store, workspace_id: i32, id: i32) -> Option<TableDefinition> {
    let mut store = read_workspace(store, workspace_id)?;
    store.tables.remove(&id)
}

pub fn get_tables(store: &Store, workspace_id: i32) -> Option<HashMap<i32, TableDefinition>> {
    read_workspace(store, workspace_id).map(|x| x.tables)
}

pub fn add_table(store: &Store, workspace_id: i32, table: TableDefinition) -> Option<i32> {
    let _lock = store.0.lock().unwrap();
    let mut workspace = read_workspace(store, workspace_id)?;
    let next_id = workspace.tables.keys().max().map_or(1, |max_id| max_id + 1);
    workspace.tables.insert(next_id, table);
    save_workspace(store, workspace_id, workspace);
    Some(next_id)
}
pub fn remove_table(store: &Store, workspace_id: i32, id: i32) -> Option<TableDefinition> {
    let _lock = store.0.lock().unwrap();
    let mut workspace = read_workspace(store, workspace_id)?;
    let removed = workspace.tables.remove(&id);
    save_workspace(store, workspace_id, workspace);
    removed
}
