use std::collections::HashMap;
use crate::db::models::TableDefinition;
use crate::db::base::{
    Store,
    SettingData,
};
pub struct SettingManager<'a> {
    pub store: &'a Store,
    pub setting: i32,
}
impl SettingManager<'_> {
    fn read_setting(&self) -> Option<SettingData> {
        let mut store = self.store.read();
        store.settings.remove(&self.setting)
    }
    fn save_setting(&self, graph: SettingData) {
        let mut store = self.store.read();
        store.settings.insert(self.setting, graph);
        self.store.save(store);
    }

    pub fn set_table(&self, id: i32, table: TableDefinition) -> Result<(), String> {
        let _lock = self.store.0.lock().unwrap();
        let mut store = self.read_setting().ok_or("Setting not found")?;
        store.tables.insert(id, table);
        self.save_setting(store);
        Ok(())
    }
    pub fn get_table(&self, id: i32) -> Option<TableDefinition> {
        let mut store = self.read_setting()?;
        store.tables.remove(&id)
    }

    pub fn get_tables(&self) -> Option<HashMap<i32, TableDefinition>> {
        self.read_setting().map(|x| x.tables)
    }

    pub fn add_table(&self, table: TableDefinition) -> Option<i32> {
        let _lock = self.store.0.lock().unwrap();
        let mut store = self.read_setting()?;
        let next_id = store.tables.keys().max().map_or(1, |max_id| max_id + 1);
        store.tables.insert(next_id, table);
        self.save_setting(store);
        Some(next_id)
    }
    pub fn remove_table(&self, id: i32) -> Option<TableDefinition> {
        let _lock = self.store.0.lock().unwrap();
        let mut store = self.read_setting()?;
        let removed = store.tables.remove(&id);
        self.save_setting(store);
        removed
    }
}
