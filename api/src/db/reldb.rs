mod workspace_manager;
mod user_workspaces_manager;
mod tables;
mod workspaces;
mod entities;
mod users;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::db::models::{
    TableDefinition,
    UserData
};
use crate::db::reldb::{
    workspaces::get_workspace,
    users::{
        add_user,
        get_users,
        get_user
    },
    workspace_manager::WorkspaceManager,
    user_workspaces_manager::UserworkspacesManager
};
use futures::executor::block_on;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
type Tables = HashMap<i32, TableDefinition>;

#[derive(Deserialize, Serialize)]
pub struct WorkspaceData {
    pub tables: Tables,
    pub user_id: i32,
    pub name: String
}

#[derive(Deserialize, Serialize)]
struct StoreData {
    workspaces : HashMap<i32, WorkspaceData>,
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
    pub fn workspace_manager(&self, workspace: i32) -> WorkspaceManager {
        WorkspaceManager {
            store: self,
            workspace
        }
    }
    pub fn reldb_conn(&self) -> &DatabaseConnection {
        &self.1
    }
    pub fn user_workspaces(&self, user_id: i32) -> UserworkspacesManager {
        UserworkspacesManager {
            store: self,
            user_id
        }
    }

    pub async fn get_workspace(&self, workspace_id: i32) -> Option<WorkspaceData> {
        get_workspace(self, workspace_id).await
    }

    pub async fn add_user(&self, name: String) -> i32 {
        add_user(self, name).await
    }
    pub async fn get_user(&self, id: i32) -> Option<UserData> {
        get_user(self, id).await
    }

    pub async fn get_users(&self) -> Vec<UserData> {
        get_users(self).await.unwrap_or_default()
    }
}


