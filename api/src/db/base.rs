mod setting_manager;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::db::models::{
    TableDefinition,
    UserData
};
use setting_manager::SettingManager;
type Tables = HashMap<i32, TableDefinition>;

#[derive(Deserialize, Serialize)]
struct SettingData {
    pub tables: Tables,
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
struct StoreData {
    settings : HashMap<i32, SettingData>,
    users: Vec<UserData>
}

#[derive(Deserialize, Serialize)]
pub struct Store(Mutex<()>);

impl Store {
    pub fn new() -> Self {
        Store(Mutex::new(()))
    }
    pub fn conn(&self, setting: i32) -> SettingManager {
        SettingManager {
            store: self,
            setting
        }
    }

    pub fn add_user(&self, name: String) -> i32 {
        let _lock = self.0.lock().unwrap();
        let mut store = self.read();
        let next_id = store.users.iter().map(|u| u.id).max().unwrap_or(0) + 1;
        store.users.push(UserData {
            id: next_id,
            name
        });
        self.save(store);
        next_id
    }
    pub fn get_user(&self, id: i32) -> Option<UserData> {
        let _lock = self.0.lock().unwrap();
        let store = self.read();
        store.users.into_iter().find(|u| u.id == id)
    }

    pub fn get_users(&self) -> Vec<UserData> {
        let _lock = self.0.lock().unwrap();
        let store = self.read();
        store.users
    }
    fn read(&self) -> StoreData {
        if let Ok(f) = std::fs::File::open("graph.yaml") {
            return serde_yaml::from_reader(f).unwrap();
        } else {
            StoreData {
                settings: HashMap::new(),
                users: Vec::new()
            }
        }
    }
    fn save(&self, store: StoreData) -> (){
        let d = serde_yaml::to_string(&store).unwrap();
        std::fs::write("graph.yaml", d).unwrap();
    }
}


