use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::db::models::{
    TableDefinition

};
#[derive(Deserialize, Serialize)]
pub struct StoreData {
    pub tables : HashMap<i32, TableDefinition>,
}

#[derive(Deserialize, Serialize)]
pub struct Store(Mutex<()>);

impl Store {
    pub fn new() -> Self {
        Store(Mutex::new(()))
    }

    fn read(&self) -> StoreData {
        let f = std::fs::File::open("graph.yaml").unwrap();
        return serde_yaml::from_reader(f).unwrap();
    }
    fn save(&self, store: StoreData) -> (){
        let d = serde_yaml::to_string(&store).unwrap();
        std::fs::write("graph.yaml", d).unwrap();
    }


    pub fn set_table(&self, id: i32, table: TableDefinition) {
        let _lock = self.0.lock().unwrap();
        let mut store = self.read();
        store.tables.insert(id, table);
        self.save(store);
    }
    pub fn get_table(&self, id: i32) -> Option<TableDefinition> {
        let mut store = self.read();
        store.tables.remove(&id)
    }

    pub fn get_tables(&self) -> HashMap<i32, TableDefinition> {
        self.read().tables
    }

    pub fn add_table(&self, table: TableDefinition) -> i32 {
        let _lock = self.0.lock().unwrap();
        let mut store = self.read();
        let next_id = store.tables.keys().max().map_or(1, |max_id| max_id + 1);
        store.tables.insert(next_id, table);
        self.save(store);
        next_id
    }
    pub fn remove_table(&self, id: i32) -> Option<TableDefinition> {
        let _lock = self.0.lock().unwrap();
        let mut store = self.read();
        let removed = store.tables.remove(&id);
        self.save(store);
        removed
    }
}

