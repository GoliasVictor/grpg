    use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema)]
struct PostWorkspace {
    pub name: String,
    pub user_id: i32

}

#[derive(Serialize, Deserialize, ToSchema)]
struct Workspace {
    pub id: i32,
    pub name: String,
    pub user_id: i32
}

#[utoipa::path(
    tags=["workspaces"],
    request_body = PostWorkspace,
    responses((status = 200, body = Workspace))
)]
#[post("/workspaces")]
pub async fn post_workspace(
    app_state: web::Data<AppState>,
    body: web::Json<PostWorkspace>,
) -> impl Responder {
    let workspace = body.into_inner();
     app_state.store.user_workspaces(workspace.user_id)
        .add_workspace(workspace.name.clone())
        .await
        .map(|id| HttpResponse::Ok().json(Workspace {
            id,
            name: workspace.name,
            user_id: workspace.user_id
        }))
        .unwrap_or_else(
            |_| HttpResponse::InternalServerError().body("Failed to create workspace")
        )
}

#[utoipa::path(
    tags=["workspaces"],
    responses(
        (status = 200, body = [Workspace]),
        (status = 404, description = "User not found")
    )
)]
#[get("/workspaces/{workspace_id}")]
pub async fn get_workspace_by_id(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let workspace_id = path.into_inner();
    if let Some(workspace) = app_state.store.get_workspace(workspace_id).await {
        HttpResponse::Ok().json(Workspace {
            id: workspace_id,
            name: workspace.name,
            user_id: workspace.user_id
        })
    } else {
        HttpResponse::NotFound().body("Workspace not found")
    }
}
#[derive(Deserialize, Serialize)]
struct GetworkspacesQuery {
    user_id: i32
}
#[utoipa::path(
    tags=["workspaces"],
    params(
        ("user_id" = i32, Query, description = "User ID")
    ),
    responses((status = 200, body = [Workspace])))]
#[get("/workspaces")]
pub async fn get_workspaces(app_state: web::Data<AppState>, query: web::Query<GetworkspacesQuery>) -> impl Responder {
    let user_id = query.user_id;
    if let Some(workspaces) = app_state.store.user_workspaces(user_id).get_workspaces().await {
        HttpResponse::Ok().json(workspaces.into_iter().map(|s| Workspace {
            id: s.0,
            name: s.1.name,
            user_id: s.1.user_id
        }).collect::<Vec<_>>())
    } else {
        HttpResponse::NotFound().body("No workspaces found for this user")
    }
}
