use super::prelude::*;
use crate::db::models::{
    TableDefinition
};
use crate::db::models::RowResponse;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TableRow {
    pub node_id: i32,
    pub label: Option<String>,
}


#[utoipa::path(
    tags=["tables"],
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    request_body = TableDefinition,
    responses(
        (status = 200, body = [RowResponse]),
        (status = 404, body = String)
    )
)]
#[put("/workspaces/{workspace_id}/table/{id}")]
pub async fn put_table(
    app_state: web::Data<AppState>,
    params: web::Json<TableDefinition>,
    path: web::Path<(i32, i32)>
) -> impl Responder {
    let (workspace_id, id) = path.into_inner();
    let table = params.into_inner();
    if let Ok(_) = app_state.store.workspace_manager(workspace_id).set_table(id, table.clone()).await {
        HttpResponse::Ok().json(app_state.graph(workspace_id).table_rows(table).await)
    } else {
        HttpResponse::NotFound().body("Table not found")
    }
}


#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct Table {
    pub id: i32,
    pub def: TableDefinition,
    pub rows: Vec<RowResponse>
}

#[utoipa::path(
    tags=["tables"],
    responses((status = 200, body = [RowResponse]))
)]
#[post("/workspaces/{workspace_id}/table")]
pub async fn post_table(
    app_state: web::Data<AppState>,
    params: web::Json<TableDefinition>,
    path: web::Path<i32>
) -> impl Responder {
    let workspace_id = path.into_inner();
    let table = params.into_inner();
    if let None = app_state.store.workspace_manager(workspace_id).add_table(table.clone()).await {
        HttpResponse::InternalServerError().body("Failed to create table")
    } else {
        HttpResponse::Ok().json(app_state.graph(workspace_id).table_rows(table).await)
    }

}

#[utoipa::path(
    tags=["tables"],
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    responses((status = 200, body = Table))
)]
#[get("/workspaces/{workspace_id}/table/{id}")]
pub async fn get_table(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (workspace_id, id) = path.into_inner();
    if let Some(table_def) = app_state.store.workspace_manager(workspace_id).get_table(id).await {
        let rows = app_state.graph(workspace_id).table_rows(table_def.clone()).await;
        HttpResponse::Ok().json( Table {
            id: id,
            def: table_def,
            rows: rows
        })
    } else {
        HttpResponse::NotFound().body("Table not found")
    }
}

#[utoipa::path(
    tags=["tables"],
    responses((status = 200, body = [Table]))
)]
#[get("/workspaces/{workspace_id}/tables")]
pub async fn get_tables(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let workspace_id = path.into_inner();
    let tables = app_state.store.workspace_manager(workspace_id).get_tables().await;
    let mut result = Vec::new();
    for (id, def) in tables.unwrap() {
        let rows = app_state.graph(workspace_id).table_rows(def.clone()).await;
        result.push(Table {
            id,
            def: def.clone(),
            rows,
        });
    }
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    tags=["tables"],
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    responses((status = 200, body = String), (status = 404, body = String))
)]
#[delete("/workspaces/{workspace_id}/tables/{id}")]
pub async fn delete_table(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (workspace_id, id) = path.into_inner();
    if app_state.store.workspace_manager(workspace_id).remove_table(id).await.is_some() {
        HttpResponse::Ok().body(format!("Table {} deleted", id))
    } else {
        HttpResponse::NotFound().body("Table not found")
    }
}
