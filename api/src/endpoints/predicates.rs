use super::prelude::*;
use crate::db::models::Predicate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostPredicate {
    pub label: String,
}

#[utoipa::path(
    tags=["predicates"],
    responses((status = 200, body = [Predicate])),
)]
#[get("/settings/{setting_id}/predicates")]
pub async fn get_predicates(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let setting_id = path.into_inner();
    HttpResponse::Ok().json(app_state.graph(setting_id).predicate_all())
}


#[utoipa::path(
    tags=["predicates"],
    request_body = PostPredicate,
    responses((status = 200, body = Predicate)),
)]
#[post("/settings/{setting_id}/predicate")]
pub async fn post_predicate(
    app_state: web::Data<AppState>,
    predicate: web::Json<PostPredicate>,
    path: web::Path<i32>,
) -> impl Responder {
    let setting_id = path.into_inner();
    HttpResponse::Ok().json(app_state.graph(setting_id).predicate_create(&predicate.label))
}
