use crate::db::base::{
    Store,
    WorkspaceData,
};
use std::collections::HashMap;
use sea_orm::{Set, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};
use crate::db::base::entities::{
    workspace,
    prelude::Workspace
};
use crate::db::models::TableDefinition;
use serde_json;

pub async fn add_workspace(store: &Store, user_id: i32, name: String) -> Result<i32, String> {
    let db = store.reldb_conn();
    let new_workspace = workspace::ActiveModel {
        name: Set(name.to_string()),
        user_id: Set(user_id),
        table_json: Set(
            serde_json::to_string(&HashMap::<i32, TableDefinition>::default())
                .unwrap()
                .to_string()
        ),
        ..Default::default()
    };
    let res = Workspace::insert(new_workspace).exec(db).await.unwrap();

    Ok(res.last_insert_id)
}
pub async fn get_workspaces(store: &Store, user_id: i32) -> Option<Vec<(i32, WorkspaceData)>> {
    let db = store.reldb_conn();

    let workspaces = Workspace::find()
            .filter(workspace::Column::UserId.eq(user_id))
            .all(db)
            .await
            .unwrap();
    Some(workspaces.into_iter().map(|w| (w.id, WorkspaceData{
        tables: serde_json::from_str(&w.table_json).unwrap(),
        user_id: w.user_id,
        name: w.name,
    })).collect::<Vec<_>>())
}

pub async fn get_workspace(store: &Store, workspace_id: i32) -> Option<WorkspaceData> {
    let db = store.reldb_conn();
    let workspace = Workspace::find_by_id(workspace_id)
        .one(db)
        .await
        .unwrap();
    workspace.map(|w| WorkspaceData {
        tables: serde_json::from_str(&w.table_json).unwrap(),
        user_id: w.user_id,
        name: w.name,
    })
}
pub async fn save_workspace(store: &Store, workspace_id: i32, graph: WorkspaceData) {
    let db = store.reldb_conn();
    Workspace::find_by_id(workspace_id)
        .one(db)
        .await
        .unwrap()
        .expect("Workspace not found");

    let updated_workspace = workspace::ActiveModel {
        id: Set(workspace_id),
        name: Set(graph.name),
        user_id: Set(graph.user_id),
        table_json: Set(serde_json::to_string(&graph.tables).unwrap()),
    };

    updated_workspace.update(db).await.unwrap();
}
