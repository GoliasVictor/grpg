use std::collections::HashMap;
use crate::db::base::{
    Store,
};
use crate::db::base::SettingData;
pub struct UserSettingsManager<'a> {
    pub store: &'a Store,
    pub user_id: i32,
}

impl UserSettingsManager<'_> {
    pub fn add_setting(&self, name: String) -> Result<i32, String> {
        let _lock = self.store.0.lock().unwrap();
        let mut store = self.store.read();
        let next_id = store.settings.keys().max().map_or(1, |max_id| max_id + 1);
        store.settings.insert(next_id, SettingData {
            tables: HashMap::new(),
            name: name,
            user_id: self.user_id,
        });
        self.store.save(store);
        Ok(next_id)
    }
    pub fn get_settings(&self) -> Option<Vec<(i32, SettingData)>> {
        let _lock = self.store.0.lock().unwrap();
        let store = self.store.read();
        Some(store.settings
            .into_iter()
            .filter(|(_,s)| s.user_id == self.user_id)
            .collect::<Vec<_>>())

    }
}
