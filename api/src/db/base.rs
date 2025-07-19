use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::db::models::{
    TableDefinition

};
type Tables = HashMap<i32, TableDefinition>;

#[derive(Deserialize, Serialize)]
pub struct SettingData {
    pub tables: Tables,
}
#[derive(Deserialize, Serialize)]
pub struct StoreData {
    pub graphs : HashMap<i32, SettingData>,
}

#[derive(Deserialize, Serialize)]
pub struct Store(Mutex<()>);
pub struct StoreConn<'a> {
    pub store: &'a Store,
    pub setting: i32,
}
impl Store {
    pub fn new() -> Self {
        Store(Mutex::new(()))
    }
    pub fn conn(&self, setting: i32) -> StoreConn {
        StoreConn {
            store: self,
            setting
        }
    }
}
impl StoreConn<'_> {
    fn read(&self) -> StoreData {
        if let Ok(f) = std::fs::File::open("graph.yaml") {
            return serde_yaml::from_reader(f).unwrap();
        } else {
            StoreData {
                graphs: HashMap::new()
            }
        }
    }
    fn save(&self, store: StoreData) -> (){
        let d = serde_yaml::to_string(&store).unwrap();
        std::fs::write("graph.yaml", d).unwrap();
    }
    fn read_graph(&self) -> SettingData {
        let mut store = self.read();
        store.graphs.remove(&self.setting).unwrap_or_else(|| SettingData {
            tables: HashMap::new()
        })
    }
    fn save_graph(&self, graph: SettingData) {
        let mut store = self.read();
        store.graphs.insert(self.setting, graph);
        self.save(store);
    }

    pub fn set_table(&self, id: i32, table: TableDefinition) {
        let _lock = self.store.0.lock().unwrap();
        let mut store = self.read_graph();
        store.tables.insert(id, table);
        self.save_graph(store);
    }
    pub fn get_table(&self, id: i32) -> Option<TableDefinition> {
        let mut store = self.read_graph();
        store.tables.remove(&id)
    }

    pub fn get_tables(&self) -> HashMap<i32, TableDefinition> {
        self.read_graph().tables
    }

    pub fn add_table(&self, table: TableDefinition) -> i32 {
        let _lock = self.store.0.lock().unwrap();
        let mut store = self.read_graph();
        let next_id = store.tables.keys().max().map_or(1, |max_id| max_id + 1);
        store.tables.insert(next_id, table);
        self.save_graph(store);
        next_id
    }
    pub fn remove_table(&self, id: i32) -> Option<TableDefinition> {
        let _lock = self.store.0.lock().unwrap();
        let mut store = self.read_graph();
        let removed = store.tables.remove(&id);
        self.save_graph(store);
        removed
    }
}

