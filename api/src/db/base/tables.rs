use std::collections::HashMap;
use crate::db::models::TableDefinition;
use crate::db::base::{
    Store,
};
use crate::db::base::settings::{
    read_setting,
    save_setting
};

pub fn set_table(store: &Store, setting_id: i32, id: i32, table: TableDefinition) -> Result<(), String> {
    let _lock = store.0.lock().unwrap();
    let mut store_data = read_setting(store, setting_id).ok_or("Setting not found")?;
    store_data.tables.insert(id, table);
    save_setting(store, setting_id, store_data);
    Ok(())
}
pub fn get_table(store: &Store, setting_id: i32, id: i32) -> Option<TableDefinition> {
    let mut store = read_setting(store, setting_id)?;
    store.tables.remove(&id)
}

pub fn get_tables(store: &Store, setting_id: i32) -> Option<HashMap<i32, TableDefinition>> {
    read_setting(store, setting_id).map(|x| x.tables)
}

pub fn add_table(store: &Store, setting_id: i32, table: TableDefinition) -> Option<i32> {
    let _lock = store.0.lock().unwrap();
    let mut setting = read_setting(store, setting_id)?;
    let next_id = setting.tables.keys().max().map_or(1, |max_id| max_id + 1);
    setting.tables.insert(next_id, table);
    save_setting(store, setting_id, setting);
    Some(next_id)
}
pub fn remove_table(store: &Store, setting_id: i32, id: i32) -> Option<TableDefinition> {
    let _lock = store.0.lock().unwrap();
    let mut setting = read_setting(store, setting_id)?;
    let removed = setting.tables.remove(&id);
    save_setting(store, setting_id, setting);
    removed
}
