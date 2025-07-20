use std::collections::HashMap;
use crate::db::base::{
    Store,
    SettingData,
};


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
        name: name,
        user_id: user_id,
    });
    store.save(store_data);
    Ok(next_id)
}
pub fn get_settings(store: &Store, user_id: i32) -> Option<Vec<(i32, SettingData)>> {
    let _lock = store.0.lock().unwrap();
    let store_data = store.read();
    Some(store_data.settings
        .into_iter()
        .filter(|(_,s)| s.user_id == user_id)
        .collect::<Vec<_>>())

}
