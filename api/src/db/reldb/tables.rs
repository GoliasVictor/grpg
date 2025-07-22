use std::collections::HashMap;
use crate::db::models::TableDefinition;
use crate::db::reldb::{
    Store,
};
use crate::db::reldb::workspaces::{
    get_workspace,
    save_workspace
};

pub async fn set_table(store: &Store, workspace_id: i32, id: i32, table: TableDefinition) -> Result<(), String> {
    let _lock = store.0.lock().unwrap();
    let mut store_data = get_workspace(store, workspace_id).await.ok_or("Workspace not found")?;
    store_data.tables.insert(id, table);
    save_workspace(store, workspace_id, store_data).await;
    Ok(())
}
pub async fn get_table(store: &Store, workspace_id: i32, id: i32) -> Option<TableDefinition> {
    let mut store = get_workspace(store, workspace_id).await?;
    store.tables.remove(&id)
}

pub async fn get_tables(store: &Store, workspace_id: i32) -> Option<HashMap<i32, TableDefinition>> {
    get_workspace(store, workspace_id).await.map(|x| x.tables)
}

pub async fn add_table(store: &Store, workspace_id: i32, table: TableDefinition) -> Option<i32> {
    let _lock = store.0.lock().unwrap();
    let mut workspace = get_workspace(store, workspace_id).await?;
    let next_id = workspace.tables.keys().max().map_or(1, |max_id| max_id + 1);
    workspace.tables.insert(next_id, table);
    save_workspace(store, workspace_id, workspace).await;
    Some(next_id)
}
pub async fn remove_table(store: &Store, workspace_id: i32, id: i32) -> Option<TableDefinition> {
    let _lock = store.0.lock().unwrap();
    let mut workspace = get_workspace(store, workspace_id).await?;
    let removed = workspace.tables.remove(&id);
    save_workspace(store, workspace_id, workspace).await;
    removed
}
