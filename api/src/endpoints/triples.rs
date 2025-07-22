use super::prelude::*;
use crate::db::models::Triple;

#[utoipa::path(
    tags=["triples"],
    request_body = Triple,
    responses((status = 200))
)]
#[post("/workspaces/{workspace_id}/triple")]
pub async fn post_triple(
    app_state: web::Data<AppState>,
    triple: web::Json<Triple>,
    path : web::Path<i32>
) -> impl Responder {
    let workspace_id = path.into_inner();
    app_state.graph(workspace_id).triple_create(triple.into_inner());
    HttpResponse::Ok()
}
#[utoipa::path(
    tags=["triples"],
    request_body = Triple,
    responses((status = 200))
)]
#[delete("/workspaces/{workspace_id}/triple")]
pub async fn delete_triple(
    app_state: web::Data<AppState>,
    triple: web::Json<Triple>,
    path: web::Path<i32>
) -> impl Responder {
    let workspace_id = path.into_inner();
    app_state.graph(workspace_id).triple_delete(triple.into_inner());
    HttpResponse::Ok()
}

#[utoipa::path(
    tags=["triples"],
    responses((status = 200, body = [Triple]))
)]
#[get("/workspaces/{workspace_id}/triples")]
pub async fn get_triples(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let workspace_id = path.into_inner();
    HttpResponse::Ok().json(app_state.graph(workspace_id).triple_all())
}
