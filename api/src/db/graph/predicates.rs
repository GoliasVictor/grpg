pub use kuzu::{
    Connection,
    Value,
};
pub use crate::db::{
    TryCast
};
pub use crate::db::models::{
    Predicate
};
use crate::db::ConnectionUtil;

pub fn predicate_all(conn: &Connection<'_>, setting: i32,) -> Vec<Predicate> {
    let result = conn.query_with_params(
        "MATCH (p:Predicate {setting: $setting}) RETURN p.id, p.label AS label",
        vec!(("setting", setting.into()))
    ).unwrap();
    let predicates : Vec<Predicate> = result
        .into_iter()
        .map(|row| Predicate {
            id: row[0].try_cast().unwrap(),
            label: row[1].try_cast().unwrap()
        })
        .collect();
    predicates
}


pub fn predicate_create(
    conn: &Connection<'_>,
    setting: i32,
    label: &str,
) -> Predicate {

    let result = conn
        .query_with_params(
            "MATCH (p:Predicate {setting: $setting}) RETURN MAX(p.id) AS id",
            vec![("setting", setting.into())]
        )
        .unwrap();
    let last_id: i32 = result
        .into_iter()
        .next()
        .and_then(|row| row[0].try_cast().ok())
        .unwrap_or(0);

    let id = last_id + 1;
    let create_result = conn
        .execute(
            &mut conn.prepare("CREATE (p:Predicate {label: $label, id: $id, setting: $setting}) RETURN p.id").unwrap(),
            vec!(
                ("label", Value::from(label)),
                ("id", Value::from(id)),
                ("setting", Value::Int64(setting as i64))
            ),
        )
        .unwrap();
    let new_id: i32 = create_result
        .into_iter()
        .next()
        .and_then(|row| row[0].try_cast().ok())
        .unwrap_or(id);

    let pred = Predicate {
        id: new_id,
        label: label.to_string(),
    };
    pred

}
