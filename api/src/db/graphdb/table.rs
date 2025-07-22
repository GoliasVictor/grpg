use std::collections::HashMap;
use crate::db::models::{
    TableDefinition,
    GraphDirection,
    RowResponse,
    CellResponse
};

pub use kuzu::{
    Connection,
    Value,
    LogicalType
};
pub use crate::db::graphdb::{
    TryCast
};

use crate::db::models::Filter;
pub async fn filter_values( conn: &Connection<'_>, workspace: i32, filter: Filter) -> Vec<i32> {
   let mut params = vec!(("workspace", Value::Int64(workspace as i64)));

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
        "(:Node { workspace: $workspace, id: $node_id })"
    } else {
        "(:Node { workspace: $workspace })"
    };
    let x =  if filter.direction.is_none()  && filter.predicate.is_none() {
        "".to_string()
    } else {
        format!("{} {}", rel_str, node_str)
    };

    let query = format!(
        "MATCH (n:Node {{workspace: $workspace}}) {} RETURN DISTINCT n.id AS id, n.label as label;",
        x
    );
    let result = conn.execute(&mut conn.prepare(&query).unwrap(), params).unwrap();

    let mut row : Vec<i32> = result
        .into_iter()
        .map(|row| row[0].try_cast().unwrap())
        .collect();

    row.sort();
    row
}
pub async fn table_rows(conn: &Connection<'_>, workspace: i32, table_def: TableDefinition) -> Vec<RowResponse> {
    let nodes_id = filter_values(&conn, workspace, table_def.filter).await;
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
        MATCH (c:Node {workspace: $workspace})-[r]->(a {workspace: $workspace})
        WHERE r.id in $out_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, 'out' as direction
        UNION ALL
        MATCH (c:Node {workspace: $workspace})<-[r]-(a {workspace: $workspace})
        WHERE r.id in $in_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, 'in' as direction
        UNION ALL
        MATCH (c:Node {workspace: $workspace})-[r]-(a {workspace: $workspace})
        WHERE r.id in $any_ids and c.id in $nids
        RETURN c.id as rnid, a.id as nid, r.id as pid, 'any' as direction
    "#;

    let query_params = vec!(
        ("workspace", Value::Int64(workspace as i64)),
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
