    use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema)]
struct PostSetting {
    pub name: String,
    pub user_id: i32

}

#[derive(Serialize, Deserialize, ToSchema)]
struct Setting {
    pub id: i32,
    pub name: String,
    pub user_id: i32
}

#[utoipa::path(request_body = PostSetting, responses((status = 200, body = Setting)))]
#[post("/settings")]
pub async fn post_setting(
    app_state: web::Data<AppState>,
    body: web::Json<PostSetting>,
) -> impl Responder {
    let setting = body.into_inner();
     app_state.store.user_settings(setting.user_id)
        .add_setting(setting.name.clone())
        .map(|id| HttpResponse::Ok().json(Setting {
            id,
            name: setting.name,
            user_id: setting.user_id
        }))
        .unwrap_or_else(
            |_| HttpResponse::InternalServerError().body("Failed to create setting")
        )
}

#[utoipa::path(responses(
    (status = 200, body = [Setting]),
    (status = 404, description = "User not found")
))]
#[get("/settings/{setting_id}")]
pub async fn get_setting_by_id(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let setting_id = path.into_inner();
    if let Some(setting) = app_state.store.get_setting(setting_id) {
        HttpResponse::Ok().json(Setting {
            id: setting_id,
            name: setting.name,
            user_id: setting.user_id
        })
    } else {
        HttpResponse::NotFound().body("Setting not found")
    }
}
#[derive(Deserialize, Serialize)]
struct GetSettingsQuery {
    user_id: i32
}
#[utoipa::path(
    params(
        ("user_id" = i32, Query, description = "User ID")
    ),
    responses((status = 200, body = [Setting])))]
#[get("/settings")]
pub async fn get_settings(app_state: web::Data<AppState>, query: web::Query<GetSettingsQuery>) -> impl Responder {
    let user_id = query.user_id;
    if let Some(settings) = app_state.store.user_settings(user_id).get_settings() {
        HttpResponse::Ok().json(settings.into_iter().map(|s| Setting {
            id: s.0,
            name: s.1.name,
            user_id: s.1.user_id
        }).collect::<Vec<_>>())
    } else {
        HttpResponse::NotFound().body("No settings found for this user")
    }
}
