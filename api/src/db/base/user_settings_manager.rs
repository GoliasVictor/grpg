use crate::db::base::{
    Store,
};
use crate::db::base::SettingData;
use crate::db::base::settings::{
    add_setting,
    get_settings
};

pub struct UserSettingsManager<'a> {
    pub store: &'a Store,
    pub user_id: i32,
}

impl UserSettingsManager<'_> {
    pub fn add_setting(&self, name: String) -> Result<i32, String> {
        add_setting(self.store, self.user_id, name)
    }
    pub fn get_settings(&self) -> Option<Vec<(i32, SettingData)>> {
        get_settings(self.store, self.user_id)
    }
}
