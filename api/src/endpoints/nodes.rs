use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::collections::HashMap;

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
#[derive(Deserialize, Serialize, ToSchema)]
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
#[derive(Deserialize, Serialize, ToSchema)]
pub struct Filter {
    pub node_id: Option<i32>,
    pub predicate: Option<i32>,
    pub direction: Option<GraphDirection>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TableRow {
    pub node_id: i32,
    pub label: Option<String>,
    pub pid: Option<i32>,
}

#[utoipa::path(
    request_body = Filter,
    responses((status = 200, body = [TableRow]))
)]
#[post("/table")]
pub async fn table(
    app_state: web::Data<AppState>,
    filter: web::Json<Filter>,
) -> impl Responder {
    let filter = filter.into_inner();
    let mut params = Vec::new();

    let triple_str = if let Some(pid) = filter.predicate {
        params.push(("pid", Value::Int64(pid as i64)));
        "[t:Triple {id: $pid}]"
    } else {
        "[t:Triple]"
    };

    let rel_str = match filter.direction {
        Some(GraphDirection::Out) => format!("-{}->", triple_str),
        Some(GraphDirection::In) => format!("<-{}-", triple_str),
        None => format!("-{}-", triple_str),
    };

    let node_str = if let Some(node_id) = filter.node_id {
        params.push(("node_id", Value::Int64(node_id as i64)));
        "(:Node { id: $node_id })"
    } else {
        "(:Node)"
    };

    let query = format!(
        "MATCH (n:Node) {} {} RETURN DISTINCT n.id AS id, t.id as pid, n.label as label;",
        rel_str, node_str
    );

    let conn = app_state.establish_connection();
    let result = conn.execute(&mut conn.prepare(&query).unwrap(), params).unwrap();

    let rows: Vec<TableRow> = result
        .into_iter()
        .map(|row| TableRow {
            node_id: row[0].try_cast().unwrap(),
            pid: row[1].try_cast().ok(),
            label: row[2].try_cast().ok(),
        })
        .collect();

    HttpResponse::Ok().json(rows)
}


#[derive(Deserialize, Serialize, ToSchema)]
pub struct ColumnFilter {
    pub direction: Option<GraphDirection>,
    pub predicate_id: Option<i32>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ColumnDefinition {
    pub id: i32,
    pub filter: ColumnFilter,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetTable {
    pub nodes_id: Vec<i32>,
    pub columns: Vec<ColumnDefinition>,
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct CellResponse {
    pub id: i32,
    pub values: Vec<i32>,
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct RowResponse {
    pub node_id: i32,
    pub columns: Vec<CellResponse>,
}

#[utoipa::path(
    request_body = GetTable,
    responses((status = 200, body = [RowResponse]))
)]
#[post("/full-table")]
pub async fn full_table(
    app_state: web::Data<AppState>,
    params: web::Json<GetTable>,
) -> impl Responder {
    let params = params.into_inner();
    if params.nodes_id.is_empty() {
        return HttpResponse::Ok().json(Vec::<RowResponse>::new());
    }

    let mut out_ids = Vec::new();
    let mut in_ids = Vec::new();
    let mut any_ids = Vec::new();

    for col in &params.columns {
        match col.filter.direction {
            Some(GraphDirection::Out) => col.filter.predicate_id.inspect(|pid| out_ids.push(*pid)),
            Some(GraphDirection::In) =>  col.filter.predicate_id.inspect(|pid| in_ids.push(*pid)),
            None => col.filter.predicate_id.inspect(|pid| any_ids.push(*pid))
        };
    }

    let query = r#"
        MATCH (c:Node)-[r]->(a)
        WHERE r.id in $out_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, 'out' as direction
        UNION ALL
        MATCH (c:Node)<-[r]-(a)
        WHERE r.id in $in_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, 'in' as direction
        UNION ALL
        MATCH (c:Node)-[r]-(a)
        WHERE r.id in $any_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, 'any' as direction
    "#;

    let query_params = vec!(
        ("nids",    Value::List(LogicalType::Int64, params.nodes_id.iter().map(|&id| Value::Int64(id as i64)).collect())),
        ("out_ids", Value::List(LogicalType::Int64, out_ids.iter().map(|&id| Value::Int64(id as i64)).collect())),
        ("in_ids",  Value::List(LogicalType::Int64, in_ids.iter().map(|&id| Value::Int64(id as i64)).collect())),
        ("any_ids", Value::List(LogicalType::Int64, any_ids.iter().map(|&id| Value::Int64(id as i64)).collect()))
    );

    let conn = app_state.establish_connection();
    let result = conn.execute(&mut conn.prepare(query).unwrap(), query_params).unwrap();

    // Group rows by rnid (node id)
    let mut grouped: HashMap<i32, Vec<(i32, i32, i32, String)>> = HashMap::new();
    for row in result {
        let rnid: i32 = row[0].try_cast().unwrap();
        let nid: i32 = row[1].try_cast().unwrap();
        let pid: i32 = row[2].try_cast().unwrap();
        let direction: String = row[3].try_cast().unwrap();
        grouped.entry(rnid).or_default().push((nid, pid, rnid, direction));
    }

    let mut rows = HashMap::new();
    for &nid in &params.nodes_id {
        rows.insert(nid, params.columns.iter().map(|col| CellResponse { id: col.id, values: Vec::new() }).collect::<Vec<_>>());
    }

    let mut response = Vec::new();
    for (&node_id, row_data) in &grouped {
        let mut columns_result = Vec::new();
        for col in &params.columns {
            let mut values : Vec<i32> = row_data.iter()
                .filter(|(_, pid, _, dir)| {
                    (col.filter.direction.is_none()
                        || col.filter.direction.as_ref()
                            .map(|d| d.to_string().to_lowercase()) == Some(dir.to_lowercase()))
                    &&
                    (col.filter.predicate_id.map(|cpid|*pid == cpid).unwrap_or(true))
                })
                .map(|(nid, _, _, _)| *nid)
                .collect();
            values.sort();
            columns_result.push(CellResponse { id: col.id, values });
        }
        response.push(RowResponse { node_id, columns: columns_result });
    }

    // Fill missing nodes with empty columns
    for &nid in &params.nodes_id {
        if !grouped.contains_key(&nid) {
            response.push(RowResponse {
                node_id: nid,
                columns: params.columns.iter().map(|col| CellResponse { id: col.id, values: Vec::new() }).collect(),
            });
        }
    }

    HttpResponse::Ok().json(params.nodes_id.iter().map( |o| response.iter().filter(|r| r.node_id == *o).next().unwrap().clone()).collect::<Vec<RowResponse>>())
}

