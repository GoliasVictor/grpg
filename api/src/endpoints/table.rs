use std::collections::HashMap;
use crate::endpoints::nodes::GraphDirection;
use super::prelude::*;
use crate::db::models::{
    TableDefinition
};

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct Filter {
    pub node_id: Option<i32>,
    pub predicate: Option<i32>,
    pub direction: Option<GraphDirection>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TableRow {
    pub node_id: i32,
    pub label: Option<String>,
}

async fn filter_values(filter: Filter, conn: &Connection<'_>) -> Vec<TableRow> {
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
    let x =  if filter.direction.is_none()  && filter.predicate.is_none() {
        "".to_string()
    } else {
        format!("{} {}", rel_str, node_str)
    };

    let query = format!(
        "MATCH (n:Node) {} RETURN DISTINCT n.id AS id, n.label as label;",
        x
    );

    let result = conn.execute(&mut conn.prepare(&query).unwrap(), params).unwrap();

    let mut row : Vec<TableRow> = result
        .into_iter()
        .map(|row| TableRow {
            node_id: row[0].try_cast().unwrap(),
            label: row[1].try_cast().ok(),
        })
        .collect();

    row.sort_by_key(|r| r.node_id);
    row
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

pub async fn table_rows(conn: &Connection<'_>, table_def: TableDefinition) -> Vec<RowResponse> {
    let nodes_id = filter_values(table_def.filter, &conn).await
        .into_iter()
        .map(|row| row.node_id)
        .collect::<Vec<_>>();
    if nodes_id.is_empty() {
        return Vec::<RowResponse>::new();
    }

    let mut out_ids = Vec::new();
    let mut in_ids = Vec::new();
    let mut any_ids = Vec::new();

    for col in &table_def.columns {
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
        ("nids",    Value::List(LogicalType::Int64, nodes_id.iter().map(|&id| Value::Int64(id as i64)).collect())),
        ("out_ids", Value::List(LogicalType::Int64, out_ids.iter().map(|&id| Value::Int64(id as i64)).collect())),
        ("in_ids",  Value::List(LogicalType::Int64, in_ids.iter().map(|&id| Value::Int64(id as i64)).collect())),
        ("any_ids", Value::List(LogicalType::Int64, any_ids.iter().map(|&id| Value::Int64(id as i64)).collect()))
    );

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
    for &nid in &nodes_id {
        rows.insert(nid, table_def.columns.iter().map(|col| CellResponse { id: col.id, values: Vec::new() }).collect::<Vec<_>>());
    }

    let mut response = Vec::new();
    for (&node_id, row_data) in &grouped {
        let mut columns_result = Vec::new();
        for col in &table_def.columns {
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
    for &nid in &nodes_id {
        if !grouped.contains_key(&nid) {
            response.push(RowResponse {
                node_id: nid,
                columns: table_def.columns.iter().map(|col| CellResponse { id: col.id, values: Vec::new() }).collect(),
            });
        }
    }

    nodes_id.iter().map( |o| response.iter().filter(|r| r.node_id == *o).next().unwrap().clone()).collect::<Vec<RowResponse>>()
}

#[utoipa::path(
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    request_body = TableDefinition,
    responses((status = 200, body = [RowResponse]))
)]
#[put("/table/{id}")]
pub async fn put_table(
    app_state: web::Data<AppState>,
    params: web::Json<TableDefinition>,
    path: web::Path<i32>
) -> impl Responder {
    let id = path.into_inner();
    let table = params.into_inner();
    let conn = app_state.establish_connection();
    app_state.store.set_table(id, table.clone());
    HttpResponse::Ok().json(table_rows(&conn, table).await)
}


#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct Table {
    pub id: i32,
    pub def: TableDefinition,
    pub rows: Vec<RowResponse>
}

#[utoipa::path(
    responses((status = 200, body = [RowResponse]))
)]
#[post("/table")]
pub async fn post_table(
    app_state: web::Data<AppState>,
    params: web::Json<TableDefinition>,
) -> impl Responder {
    let table = params.into_inner();
    app_state.store.add_table(table.clone());
    let conn = app_state.establish_connection();
    HttpResponse::Ok().json(table_rows(&conn, table).await)
}

#[utoipa::path(
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    responses((status = 200, body = Table))
)]
#[get("/table/{id}")]
pub async fn get_table(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    if let Some(table_def) = app_state.store.get_table(id) {
        let conn = app_state.establish_connection();
        let rows = table_rows(&conn, table_def.clone()).await;
        HttpResponse::Ok().json( Table {
            id: id,
            def: table_def.clone(),
            rows: rows
        })
    } else {
        HttpResponse::NotFound().body("Table not found")
    }
}

#[utoipa::path(
    responses((status = 200, body = [Table]))
)]
#[get("/tables")]
pub async fn get_tables(
    app_state: web::Data<AppState>,
) -> impl Responder {
    let tables = app_state.store.get_tables();
    let conn = app_state.establish_connection();
    let mut result = Vec::new();
    for (id, def) in tables {
        let rows = table_rows(&conn, def.clone()).await;
        result.push(Table {
            id,
            def: def.clone(),
            rows,
        });
    }
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    params(
        ("id" = i32, Path, description = "Table ID")
    ),
    responses((status = 200, body = String), (status = 404, body = String))
)]
#[delete("/tables/{id}")]
pub async fn delete_table(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    if app_state.store.remove_table(id).is_some() {
        HttpResponse::Ok().body(format!("Table {} deleted", id))
    } else {
        HttpResponse::NotFound().body("Table not found")
    }
}
