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
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    request_body = TableDefinition,
    responses(
        (status = 200, body = [RowResponse]),
        (status = 404, body = String)
    )
)]
#[put("/settings/{setting_id}/table/{id}")]
pub async fn put_table(
    app_state: web::Data<AppState>,
    params: web::Json<TableDefinition>,
    path: web::Path<(i32, i32)>
) -> impl Responder {
    let (setting_id, id) = path.into_inner();
    let table = params.into_inner();
    if let Ok(_) = app_state.store.conn(setting_id).set_table(id, table.clone()) {
        HttpResponse::Ok().json(app_state.graph(setting_id).table_rows(table).await)
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
    responses((status = 200, body = [RowResponse]))
)]
#[post("/settings/{setting_id}/table")]
pub async fn post_table(
    app_state: web::Data<AppState>,
    params: web::Json<TableDefinition>,
    path: web::Path<i32>
) -> impl Responder {
    let setting_id = path.into_inner();
    let table = params.into_inner();
    if let None = app_state.store.conn(setting_id).add_table(table.clone()) {
        HttpResponse::InternalServerError().body("Failed to create table")
    } else {
        HttpResponse::Ok().json(app_state.graph(setting_id).table_rows(table).await)
    }

}

#[utoipa::path(
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    responses((status = 200, body = Table))
)]
#[get("/settings/{setting_id}/table/{id}")]
pub async fn get_table(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (setting_id, id) = path.into_inner();
    if let Some(table_def) = app_state.store.conn(setting_id).get_table(id) {
        let rows = app_state.graph(setting_id).table_rows(table_def.clone()).await;
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
    responses((status = 200, body = [Table]))
)]
#[get("/settings/{setting_id}/tables")]
pub async fn get_tables(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let setting_id = path.into_inner();
    let tables = app_state.store.conn(setting_id).get_tables();
    let mut result = Vec::new();
    for (id, def) in tables.unwrap() {
        let rows = app_state.graph(setting_id).table_rows(def.clone()).await;
        result.push(Table {
            id,
            def: def.clone(),
            rows,
        });
    }
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    responses((status = 200, body = String), (status = 404, body = String))
)]
#[delete("/settings/{setting_id}/tables/{id}")]
pub async fn delete_table(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (setting_id, id) = path.into_inner();
    if app_state.store.conn(setting_id).remove_table(id).is_some() {
        HttpResponse::Ok().body(format!("Table {} deleted", id))
    } else {
        HttpResponse::NotFound().body("Table not found")
    }
}
