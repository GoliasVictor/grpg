use super::prelude::*;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct NodeResponse {
    pub node_id: i32,
}
#[derive(Deserialize, Serialize, IntoParams)]
pub struct NewNode {
    pub label: String
}


#[utoipa::path(
    params(NewNode),
    responses((status = 200, body = NodeResponse))
)]
#[post("/node")]
pub async fn post_node(app_state: web::Data<AppState>, label: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let label = match label.get("label") {
        Some(l) => l,
        None => return HttpResponse::BadRequest().body("Missing label parameter"),
    };

    let conn = app_state.establish_connection();

    let result = conn.query("MATCH (n:Node) RETURN MAX(n.id) AS id;").unwrap();
    let last_id : i32 = result.single().unwrap_or(0);

    let query = r#"
        CREATE (n:Node {id: $id, label: $label})
        RETURN n.id;
    "#;

    let params = vec!(
        ("id", Value::Int64((last_id + 1) as i64)),
        ("label", Value::String(label.clone()))
    );

    let result = conn.execute(&mut conn.prepare(query).unwrap(), params).unwrap();
    let node_id : i32 = result.single().unwrap();

    HttpResponse::Ok().json(NodeResponse { node_id })
}


#[derive(Deserialize, Serialize, ToSchema)]
pub struct Node {
    pub node_id: i32,
    pub label: String,
}

#[utoipa::path(
    responses((status = 200, body = [Node]))
)]
#[get("/node")]
pub async fn get_node(app_state: web::Data<AppState>) -> impl Responder {
    let conn = app_state.establish_connection();
    let result = conn.query("MATCH (n:Node) RETURN n.id AS id, n.label as label;").unwrap();

    let nodes: Vec<Node> = result
        .into_iter()
        .map(|row| Node {
            node_id: row[0].try_cast().unwrap(),
            label: row[1].try_cast().unwrap_or_else(|_| "".to_string()),
        })
        .collect();

    HttpResponse::Ok().json(nodes)
}

#[utoipa::path(
    params(
        ("node_id" = i32, Path, description = "ID do nó a ser atualizado"),
        ("label" = String, Query, description = "Novo label do nó")
    ),
    responses((status = 200, body = Node))
)]
#[put("/node/{node_id}")]
pub async fn put_node(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    label: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let node_id = path.into_inner();
    let label = match label.get("label") {
        Some(l) => l,
        None => return HttpResponse::BadRequest().body("Missing label parameter"),
    };

    let conn = app_state.establish_connection();

    let query = r#"
        MATCH (n:Node {id: $id})
        SET n.label = $label
        RETURN n.label;
    "#;

    let params = vec![
        ("id", Value::Int64(node_id as i64)),
        ("label", Value::String(label.clone())),
    ];

    let result = conn.execute(&mut conn.prepare(query).unwrap(), params).unwrap();
    let new_label: String = result.single().unwrap_or_else(|| "".to_string());

    HttpResponse::Ok().json(Node {
        node_id,
        label: new_label,
    })
}



#[utoipa::path(
    params(
        ("node_id" = i32, Path, description = "ID do nó a ser deletado")
    ),
    responses((status = 200, body = NodeResponse))
)]
#[delete("/node/{node_id}")]
pub async fn delete_node(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let node_id = path.into_inner();

    let conn = app_state.establish_connection();

    let query = r#"
        MATCH (n:Node {id: $id}) DETACH DELETE n;
    "#;

    let params = vec![
        ("id", Value::Int64(node_id as i64))
    ];

    let _ = conn.execute(&mut conn.prepare(query).unwrap(), params);

    HttpResponse::Ok().json(NodeResponse { node_id })
}
#[derive(Deserialize, Serialize, ToSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GraphDirection {
    In,
    Out,
}

impl GraphDirection {
    pub fn to_string(&self) -> String {
        match self {
            GraphDirection::In => "in".to_string(),
            GraphDirection::Out => "out".to_string(),
        }
    }
}
