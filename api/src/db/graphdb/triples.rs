use crate::db::TryCast;
use crate::db::models::Triple;
use kuzu::{
    Value,
    Connection
};
use crate::db::ConnectionUtil;

pub fn triple_create(
    conn: &Connection<'_>,
    workspace: i32,
    triple: Triple,
) -> () {
    let query =
        r#"
        MATCH (n1:Node {workspace: $workspace}), (n2:Node {workspace: $workspace})
        WHERE n1.id = $id1 AND n2.id = $id2
        CREATE (n1)-[t:Triple { id: $pid }]->(n2)
        RETURN t.id;
    "#;

    let params = vec![
        ("workspace", Value::Int64(workspace as i64)),
        ("id1", Value::Int64(triple.subject_id as i64)),
        ("pid", Value::Int64(triple.predicate_id as i64)),
        ("id2", Value::Int64(triple.object_id as i64))
    ];

    let _result = conn.execute(&mut conn.prepare(query).unwrap(), params).unwrap();
}


pub fn triple_delete(
    conn: &Connection<'_>,
    workspace: i32,
    triple: Triple,
) -> () {
    let query =
        r#"
        MATCH (n1:Node {id: $id1, workspace: $workspace})-[t:Triple {id: $pid}]->(n2:Node {id: $id2, workspace: $workspace})
        DELETE t;
    "#;

    let params = vec![
        ("workspace", Value::Int64(workspace as i64)),
        ("id1", Value::Int64(triple.subject_id as i64)),
        ("pid", Value::Int64(triple.predicate_id as i64)),
        ("id2", Value::Int64(triple.object_id as i64))
    ];

    let _ = conn.execute(&mut conn.prepare(query).unwrap(), params);
}

pub fn triple_all(
    conn: &Connection<'_>,
    workspace: i32
) -> Vec<Triple> {
    let query =
        r#"
        MATCH (n1:Node {workspace: $workspace})-[t:Triple]->(n2:Node{workspace: $workspace})
        RETURN n1.id AS subject_id, t.id AS predicate_id, n2.id AS object_id;
    "#;

    let result = conn.query_with_params(
        query,
        vec![("workspace", workspace.into())]
    ).unwrap();

    let triples: Vec<Triple> = result
        .into_iter()
        .map(|row| {
            Some(Triple {
                subject_id: row[0].try_cast().ok()?,
                predicate_id: row[1].try_cast().ok()?,
                object_id: row[2].try_cast().ok()?,
            })
        })
        .filter_map(|d|d)
        .collect();
    triples
}
