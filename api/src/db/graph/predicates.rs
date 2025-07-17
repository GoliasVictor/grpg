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

pub fn predicate_all(conn: &Connection<'_>) -> Vec<Predicate> {
    let result = conn.query("MATCH (p:Predicate) RETURN p.id, p.label AS label").unwrap();
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
    label: &str,
) -> Predicate {

    let result = conn
        .query("MATCH (p:Predicate) RETURN MAX(p.id) AS id")
        .unwrap();
    let last_id: i32 = result
        .into_iter()
        .next()
        .and_then(|row| row[0].try_cast().ok())
        .unwrap_or(0);

    let id = last_id + 1;
    let create_result = conn
        .execute(
            &mut conn.prepare("CREATE (p:Predicate {label: $label, id: $id}) RETURN p.id").unwrap(),
            vec!(("label", Value::from(label)), ("id", Value::from(id))),
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
