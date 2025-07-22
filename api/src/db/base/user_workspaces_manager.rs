use crate::db::base::{
    Store,
};
use crate::db::base::WorkspaceData;
use crate::db::base::workspaces::{
    add_workspace,
    get_workspaces
};

pub struct UserworkspacesManager<'a> {
    pub store: &'a Store,
    pub user_id: i32,
}

impl UserworkspacesManager<'_> {
    pub async fn add_workspace(&self, name: String) -> Result<i32, String> {
        add_workspace(self.store, self.user_id, name).await
    }
    pub async fn get_workspaces(&self) -> Option<Vec<(i32, WorkspaceData)>> {
        get_workspaces(self.store, self.user_id).await
    }
}
