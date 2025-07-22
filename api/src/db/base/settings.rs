use std::collections::HashMap;
use crate::db::base::{
    Store,
    SettingData,
};

use sea_orm::{Database, Set, EntityTrait, QueryFilter, ColumnTrait};
use crate::db::base::entities::{
    workspace,
    prelude::Workspace
};
use futures::executor::block_on;

const DATABASE_URL: &str = "sqlite:./db.sqlite?mode=rwc";

pub async fn db_add_setting(_store: &Store, user_id: i32, name: String) -> Result<i32, String> {
    let db = &Database::connect(DATABASE_URL).await.unwrap();
    let new_workspace = workspace::ActiveModel {
        name: Set(name.to_string()),
        user_id: Set(user_id),
        ..Default::default()
    };
    let res = Workspace::insert(new_workspace).exec(db).await.unwrap();

    Ok(res.last_insert_id)
}
pub async fn db_get_settings(_store: &Store, user_id: i32) -> Option<Vec<(i32, SettingData)>> {
    let db = &Database::connect(DATABASE_URL).await.unwrap();

    let workspaces = Workspace::find()
            .filter(workspace::Column::UserId.eq(user_id))
            .all(db)
            .await
            .unwrap();
    Some(workspaces.into_iter().map(|w| (w.id, SettingData{
        tables: HashMap::new(), // Assuming no tables are associated initially
        user_id: w.user_id,
        name: w.name,
    })).collect::<Vec<_>>())
}

pub fn read_setting(store: &Store, setting_id: i32) -> Option<SettingData> {
    let mut store_data = store.read();
    store_data.settings.remove(&setting_id)
}
pub fn save_setting(store: &Store, setting_id: i32, graph: SettingData) {
    let mut store_data = store.read();
    store_data.settings.insert(setting_id, graph);
    store.save(store_data);
}

pub fn add_setting(store: &Store, user_id: i32, name: String) -> Result<i32, String> {
    let _lock = store.0.lock().unwrap();
    let mut store_data = store.read();
    let next_id = store_data.settings.keys().max().map_or(1, |max_id| max_id + 1);
    store_data.settings.insert(next_id, SettingData {
        tables: HashMap::new(),
        name: name.clone(),
        user_id: user_id,
    });
    store.save(store_data);
    block_on(db_add_setting(store, user_id, name)).unwrap();
    Ok(next_id)
}
pub fn get_settings(store: &Store, user_id: i32) -> Option<Vec<(i32, SettingData)>> {
    block_on(db_get_settings(store, user_id))
}
