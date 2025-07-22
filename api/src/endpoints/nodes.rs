use super::prelude::*;
use crate::db::models::{
    Node
};
#[derive(Deserialize, Serialize, ToSchema)]
pub struct NodeResponse {
    pub node_id: i32,
}
#[derive(Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in=Query)]
pub struct NewNode {
    pub label: String
}

#[utoipa::path(
    tags=["nodes"],
    responses((status = 200, body = NodeResponse))
)]
#[post("/workspaces/{workspace_id}/node")]
pub async fn post_node(app_state: web::Data<AppState>, new_label: web::Json<NewNode>, path: web::Path<i32>) -> impl Responder {
    let workspace_id = path.into_inner();
    let label = new_label.into_inner().label;
    HttpResponse::Ok().json(NodeResponse {
        node_id: app_state.graph(workspace_id).node_create(label.clone())
     })
}

#[utoipa::path(
    tags=["nodes"],
    responses((status = 200, body = [Node]))
)]
#[get("/workspaces/{workspace_id}/node")]
pub async fn get_node(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let workspace_id = path.into_inner();
    HttpResponse::Ok().json(app_state.graph(workspace_id).node_all())
}

#[utoipa::path(
    tags=["nodes"],
    params(
        ("node_id" = i32, Path, description = "ID do nó a ser atualizado"),
        ("label" = String, Query, description = "Novo label do nó")
    ),
    responses((status = 200, body = Node))
)]
#[put("/workspaces/{workspace_id}/node/{node_id}")]
pub async fn put_node(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    mut label: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let (workspace_id, node_id) = path.into_inner();
    let label = match label.remove("label") {
        Some(l) => l,
        None => return HttpResponse::BadRequest().body("Missing label parameter"),
    };

    HttpResponse::Ok().json(app_state.graph(workspace_id).node_update(node_id, label))
}



#[utoipa::path(
    tags=["nodes"],
    params(
        ("node_id" = i32, Path, description = "ID do nó a ser deletado")
    ),
    responses((status = 200, body = NodeResponse))
)]
#[delete("/workspaces/{workspace_id}/node/{node_id}")]
pub async fn delete_node(app_state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let (workspace_id, node_id) = path.into_inner();

    app_state.graph(workspace_id).node_delete(node_id);

    HttpResponse::Ok().json(NodeResponse { node_id })
}
