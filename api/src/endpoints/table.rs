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
    responses((status = 200, body = [RowResponse]))
)]
#[put("/table/{id}")]
pub async fn put_table(
    app_state: web::Data<AppState>,
    params: web::Json<TableDefinition>,
    path: web::Path<i32>
) -> impl Responder {
    let id = path.into_inner();
    let table = params.into_inner();
    app_state.store.set_table(id, table.clone());
    HttpResponse::Ok().json(app_state.graph().table_rows(table).await)
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
#[post("/table")]
pub async fn post_table(
    app_state: web::Data<AppState>,
    params: web::Json<TableDefinition>,
) -> impl Responder {
    let table = params.into_inner();
    app_state.store.add_table(table.clone());
    HttpResponse::Ok().json(app_state.graph().table_rows(table).await)
}

#[utoipa::path(
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    responses((status = 200, body = Table))
)]
#[get("/table/{id}")]
pub async fn get_table(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    if let Some(table_def) = app_state.store.get_table(id) {
        let rows = app_state.graph().table_rows(table_def.clone()).await;
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
#[get("/tables")]
pub async fn get_tables(
    app_state: web::Data<AppState>,
) -> impl Responder {
    let tables = app_state.store.get_tables();
    let mut result = Vec::new();
    for (id, def) in tables {
        let rows = app_state.graph().table_rows(def.clone()).await;
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
#[delete("/tables/{id}")]
pub async fn delete_table(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    if app_state.store.remove_table(id).is_some() {
        HttpResponse::Ok().body(format!("Table {} deleted", id))
    } else {
        HttpResponse::NotFound().body("Table not found")
    }
}
