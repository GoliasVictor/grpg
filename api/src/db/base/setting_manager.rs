use std::collections::HashMap;
use crate::db::models::TableDefinition;
use crate::db::base::{
    Store,
};
pub struct SettingManager<'a> {
    pub store: &'a Store,
    pub setting: i32,
}
use crate::db::base::tables::{
    set_table,
    get_table,
    get_tables,
    add_table,
    remove_table
};
impl SettingManager<'_> {
    pub fn set_table(&self, id: i32, table: TableDefinition) -> Result<(), String> {
        set_table(self.store, self.setting, id, table)
    }
    pub fn get_table(&self, id: i32) -> Option<TableDefinition> {
        get_table(self.store, self.setting, id)
    }
    pub fn get_tables(&self) -> Option<HashMap<i32, TableDefinition>> {
        get_tables(self.store, self.setting)
    }
    pub fn add_table(&self, table: TableDefinition) -> Option<i32> {
        add_table(self.store, self.setting, table)
    }
    pub fn remove_table(&self, id: i32) -> Option<TableDefinition> {
        remove_table(self.store, self.setting, id)
    }
}
