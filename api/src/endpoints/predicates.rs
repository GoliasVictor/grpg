use super::prelude::*;
use crate::db::models::Predicate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostPredicate {
    pub label: String,
}

#[utoipa::path(
    responses((status = 200, body = [Predicate])),
)]
#[get("/predicates")]
pub async fn get_predicates(app_state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_state.graph().predicate_all())
}


#[utoipa::path(
    request_body = PostPredicate,
    responses((status = 200, body = Predicate)),
)]
#[post("/predicate")]
pub async fn post_predicate(
    app_state: web::Data<AppState>,
    predicate: web::Json<PostPredicate>,
) -> impl Responder {
    HttpResponse::Ok().json(app_state.graph().predicate_create(&predicate.label))
}
