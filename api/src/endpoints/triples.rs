use super::prelude::*;


#[derive(Deserialize, Serialize, ToSchema)]
pub struct Triple {
    pub subject_id: i32,
    pub predicate_id: i32,
    pub object_id: i32,
}

#[utoipa::path(
    request_body = Triple,
    responses((status = 200))
)]
#[post("/triple")]
pub async fn post_triple(
    app_state: web::Data<AppState>,
    triple: web::Json<Triple>,
) -> impl Responder {
    let conn = app_state.establish_connection();

    let query = r#"
        MATCH (n1:Node), (n2:Node)
        WHERE n1.id = $id1 AND n2.id = $id2
        CREATE (n1)-[t:Triple { id: $pid }]->(n2)
        RETURN t.id;
    "#;

    let params = vec![
        ("id1", Value::Int64(triple.subject_id as i64)),
        ("pid", Value::Int64(triple.predicate_id as i64)),
        ("id2", Value::Int64(triple.object_id as i64)),
    ];

    let _result = conn.execute(&mut conn.prepare(query).unwrap(), params).unwrap();

    HttpResponse::Ok()
}
#[utoipa::path(
    request_body = Triple,
    responses((status = 200))
)]
#[delete("/triple")]
pub async fn delete_triple(
    app_state: web::Data<AppState>,
    triple: web::Json<Triple>,
) -> impl Responder {
    let conn = app_state.establish_connection();

    let query = r#"
        MATCH (n1:Node {id: $id1})-[t:Triple {id: $pid}]->(n2:Node {id: $id2})
        DELETE t;
    "#;

    let params = vec![
        ("id1", Value::Int64(triple.subject_id as i64)),
        ("pid", Value::Int64(triple.predicate_id as i64)),
        ("id2", Value::Int64(triple.object_id as i64)),
    ];

    let _ = conn.execute(&mut conn.prepare(query).unwrap(), params);
    HttpResponse::Ok()
}
