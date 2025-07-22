mod setting_manager;
mod user_settings_manager;
mod tables;
mod settings;
mod entities;
mod users;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::db::models::{
    TableDefinition,
    UserData
};
use crate::db::base::{
    settings::get_setting,
    users::{
        add_user,
        get_users,
        get_user
    },
    setting_manager::SettingManager,
    user_settings_manager::UserSettingsManager
};
use futures::executor::block_on;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
type Tables = HashMap<i32, TableDefinition>;

#[derive(Deserialize, Serialize)]
pub struct SettingData {
    pub tables: Tables,
    pub user_id: i32,
    pub name: String
}

#[derive(Deserialize, Serialize)]
struct StoreData {
    settings : HashMap<i32, SettingData>,
    users: Vec<UserData>
}

pub struct Store(Mutex<()>, DatabaseConnection);
const DATABASE_URL: &str = "sqlite:./db.sqlite?mode=rwc";
impl Store {
    pub fn new() -> Self {
        Store(Mutex::new(()), block_on(async {
            Database::connect(DATABASE_URL).await.unwrap()
        }))
    }
    pub fn conn(&self, setting: i32) -> SettingManager {
        SettingManager {
            store: self,
            setting
        }
    }
    pub fn reldb_conn(&self) -> &DatabaseConnection {
        &self.1
    }
    pub fn user_settings(&self, user_id: i32) -> UserSettingsManager {
        UserSettingsManager {
            store: self,
            user_id
        }
    }

    pub fn get_setting(&self, setting_id: i32) -> Option<SettingData> {
        get_setting(self, setting_id)
    }

    pub fn add_user(&self, name: String) -> i32 {
        block_on(add_user(self, name))
    }
    pub fn get_user(&self, id: i32) -> Option<UserData> {
        block_on(get_user(self, id))
    }

    pub fn get_users(&self) -> Vec<UserData> {
        block_on(get_users(self)).unwrap_or_default()
    }
}


