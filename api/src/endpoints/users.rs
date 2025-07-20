use super::prelude::*;
use crate::db::models::UserData;

#[derive(Serialize, Deserialize, ToSchema)]
struct PostUser {
    pub username: String
}
#[utoipa::path(request_body = PostUser, responses((status = 200, body = UserData)))]
#[post("/users")]
pub async fn post_user(
    app_state: web::Data<AppState>,
    body: web::Json<PostUser>,
) -> impl Responder {
    let user = body.into_inner();
    let id = app_state.store.add_user(user.username.clone());
    HttpResponse::Ok().json(UserData { id, name: user.username })
}

#[utoipa::path(responses(
    (status = 200, body = [UserData]),
    (status = 404, description = "User not found")
))]
#[get("/users/{user_id}")]
pub async fn get_user_by_id(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    if let Some(user) = app_state.store.get_user(user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[utoipa::path(responses((status = 200, body = [UserData])))]
#[get("/users")]
pub async fn get_users(app_state: web::Data<AppState>) -> impl Responder {
    let users = app_state.store.get_users();
    HttpResponse::Ok().json(users)
}
