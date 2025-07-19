use crate::db::TryCast;
use kuzu::{
    Value,
    Connection
};
use crate::db::QueryResultUtil;
use crate::db::models::Node;
use crate::db::ConnectionUtil;

pub fn node_create(
    conn: &Connection<'_>,
    setting: i32,
    label: String
)  -> i32 {

    let result = conn.query_with_params(
        "MATCH (n:Node {setting: $setting}) RETURN MAX(n.id) AS id;",
        vec![("setting", setting.into())]
    ).unwrap();
    let last_id : i32 = result.single().unwrap_or(0);

    let query = r#"
        CREATE (n:Node {setting: $setting, id: $id, label: $label})
        RETURN n.id;
    "#;

    let params = vec!(
        ("setting", Value::Int64(setting as i64)),
        ("id", Value::Int64((last_id + 1) as i64)),
        ("label", Value::String(label.clone()))
    );

    let result = conn.execute(&mut conn.prepare(query).unwrap(), params).unwrap();
    let node_id : i32 = result.single().unwrap();
    return node_id;
}

pub fn node_all(
    conn: &Connection<'_>,
    setting: i32
) -> Vec<Node> {
    let result = conn.query_with_params(
        "MATCH (n:Node {setting: $setting}) RETURN n.id AS id, n.label as label;",
        vec![("setting", setting.into())]
    ).unwrap();

    let nodes: Vec<Node> = result
        .into_iter()
        .map(|row| Node {
            node_id: row[0].try_cast().unwrap(),
            label: row[1].try_cast().unwrap_or_else(|_| "".to_string()),
        })
        .collect();
    return nodes;
}

pub fn node_update(
    conn: &Connection<'_>,
    setting: i32,
    node_id: i32,
    label: String
) -> Node {
    let query = r#"
        MATCH (n:Node {id: $id, setting: $setting})
        SET n.label = $label
        RETURN n.label;
    "#;

    let params = vec![
        ("id", Value::Int64(node_id as i64)),
        ("label", Value::String(label.clone())),
        ("setting", Value::Int64(setting as i64))
    ];

    let result = conn.execute(&mut conn.prepare(query).unwrap(), params).unwrap();
    let new_label: String = result.single().unwrap_or_else(|| "".to_string());

    Node {
        node_id,
        label: new_label,
    }
}

pub fn node_delete(
    conn: &Connection<'_>,
    setting: i32,
    node_id: i32,
) -> () {
    let query = r#"
        MATCH (n:Node {id: $id, setting: $setting}) DETACH DELETE n;
    "#;

    let params = vec![
        ("id", Value::Int64(node_id as i64)),
        ("setting", Value::Int64(setting as i64))
    ];

    let _ = conn.execute(&mut conn.prepare(query).unwrap(), params);
}
