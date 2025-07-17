use super::prelude::*;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Triple {
    pub subject_id: i32,
    pub predicate_id: i32,
    pub object_id: i32,
}

#[utoipa::path(request_body = Triple, responses((status = 200)))]
#[post("/triple")]
pub async fn post_triple(
    app_state: web::Data<AppState>,
    triple: web::Json<Triple>
) -> impl Responder {
    app_state.graph().triple_create(triple.into_inner());
    HttpResponse::Ok()
}
#[utoipa::path(request_body = Triple, responses((status = 200)))]
#[delete("/triple")]
pub async fn delete_triple(
    app_state: web::Data<AppState>,
    triple: web::Json<Triple>
) -> impl Responder {
    app_state.graph().triple_delete(triple.into_inner());
    HttpResponse::Ok()
}

#[utoipa::path(responses((status = 200, body = [Triple])))]
#[get("/triples")]
pub async fn get_triples(app_state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_state.graph().triple_all())
}
