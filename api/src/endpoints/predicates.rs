use super::prelude::*;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Predicate {
    pub id: i32,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostPredicate {
    pub label: String,
}

#[utoipa::path(
    responses((status = 200)),
)]
#[get("/predicates")]
pub async fn get_predicates(app_state: web::Data<AppState>) -> impl Responder {
    let conn = app_state.establish_connection();
    let result = conn.query("MATCH (p:Predicate) RETURN p.id, p.label AS label").unwrap();
    let predicates : Vec<Predicate> = result
        .into_iter()
        .map(|row| Predicate {
            id: row[0].try_cast().unwrap(),
            label: row[1].try_cast().unwrap()
        })
        .collect();
    HttpResponse::Ok().json(predicates)
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
    let conn = app_state.establish_connection();

    let result = conn
        .query("MATCH (p:Predicate) RETURN MAX(p.id) AS id")
        .unwrap();
    let last_id: i32 = result
        .into_iter()
        .next()
        .and_then(|row| row[0].try_cast().ok())
        .unwrap_or(0);

    // Insert the new predicate
    let id = last_id + 1;
    let create_result = conn
        .execute(
            &mut conn.prepare("CREATE (p:Predicate {label: $label, id: $id}) RETURN p.id").unwrap(),
            vec!(("label", Value::from(predicate.label.clone())), ("id", Value::from(id))),
        )
        .unwrap();
    let new_id: i32 = create_result
        .into_iter()
        .next()
        .and_then(|row| row[0].try_cast().ok())
        .unwrap_or(id);

    let pred = Predicate {
        id: new_id,
        label: predicate.label.clone(),
    };

    HttpResponse::Ok().json(pred)
}
