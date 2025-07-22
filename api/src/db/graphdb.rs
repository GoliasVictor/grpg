mod triples;
mod nodes;
mod predicates;
use crate::db::models::Triple;
use crate::db::models::{
    Node,
    Predicate,
    TableDefinition,
    RowResponse
};
use kuzu::{ Connection, Value, QueryResult, Database,  SystemConfig };
mod table;
pub struct GraphManager<'a> {
    pub conn: Connection<'a>,
    pub workspace: i32
}

impl<'a> GraphManager<'a> {
    pub fn triple_create(&self, triple: Triple) {
        triples::triple_create(&self.conn, self.workspace, triple)
    }
    pub fn triple_delete(&self, triple: Triple) {
        triples::triple_delete(&self.conn, self.workspace, triple)
    }
    pub fn triple_all(&self) -> Vec<Triple> {
        triples::triple_all(&self.conn, self.workspace)
    }
    pub fn node_create(&self, label: String) -> i32 {
        nodes::node_create(&self.conn, self.workspace, label)
    }
    pub fn node_all(&self,) -> Vec<Node> {
        nodes::node_all(&self.conn, self.workspace)
    }
    pub fn node_update(&self, node_id: i32, label: String) -> Node {
        nodes::node_update(&self.conn, self.workspace, node_id, label)
    }
    pub fn node_delete(&self, node_id: i32) {
        nodes::node_delete(&self.conn, self.workspace, node_id)
    }
    pub fn predicate_all(&self) -> Vec<Predicate> {
        predicates::predicate_all(&self.conn, self.workspace)
    }
    pub fn predicate_create(&self, label: &str ) -> Predicate {
        predicates::predicate_create(&self.conn, self.workspace, label)
    }
    pub async fn table_rows(&self, table_def: TableDefinition) -> Vec<RowResponse> {
        table::table_rows(&self.conn, self.workspace, table_def).await
    }
}

pub trait TryCast<T> {
    type Error;
    fn try_cast(&self) -> Result<T, Self::Error>;
}
impl TryCast<i32> for Value {
    type Error = String;

    fn try_cast(&self) -> Result<i32, Self::Error> {
        match self {
            Value::Int64(i) => Ok(*i as i32),
            _ => Err("Cannot convert to i32".to_string()),
        }
    }
}

impl TryCast<String> for Value {
    type Error = String;

    fn try_cast(&self) -> Result<String, Self::Error> {
        match self {
            Value::String(s) => Ok(s.clone()),
            _ => Err("Cannot convert to i32".to_string()),
        }
    }
}

pub trait TryFromValue: Sized {
    fn from_value(value: &Value) -> Option<Self>;
}

impl<T> TryFromValue for T where Value: TryCast<T> {
    fn from_value(value: &Value) -> Option<T> {
        value.try_cast().ok()
    }
}

pub trait QueryResultUtil {
    fn single<K: TryFromValue>(self) -> Option<K>;

}
pub trait ConnectionUtil {
    fn query_with_params(
        &self,
        query: &str,
        params: Vec<(&str, QueryValue)>
    ) -> Result<QueryResult<'_>, kuzu::Error>;
}
impl QueryResultUtil for QueryResult<'_> {
    fn single<K: TryFromValue>(self) -> Option<K> {
        let row = self.into_iter().next()?;
        let value = row.get(0)?;
        K::from_value(value)
    }
}
pub struct QueryValue(Value);
impl Into<QueryValue> for i32 {
    fn into(self) -> QueryValue {
        QueryValue(Value::Int64(self as i64))
    }
}
impl Into<QueryValue> for String {
    fn into(self) -> QueryValue {
        QueryValue(Value::String(self))
    }
}
impl Into<QueryValue> for &str {
    fn into(self) -> QueryValue {
        QueryValue(Value::String(self.to_string()))
    }
}

impl ConnectionUtil for Connection<'_> {
    fn query_with_params(
        &self,
        query: &str,
        params: Vec<(&str, QueryValue)>
    ) -> Result<QueryResult<'_>, kuzu::Error> {
        let params: Vec<(&str, Value)> = params.into_iter()
            .map(|(name, value)| (name, value.0))
            .collect();
        self.execute(&mut self.prepare(query).unwrap(), params)
    }
}


pub struct GraphDatabase {
    db: Database,
}

impl GraphDatabase {
    pub fn new(path: &str) -> Self {
        let db = Database::new(path, SystemConfig::default()).unwrap();
        Self { db }
    }

    pub fn connection(&self) -> Connection<'_> {
        Connection::new(&self.db).unwrap()
    }
    pub fn startup(&self) {
        let conn = self.connection();
        let _ = conn.query(
            "CREATE NODE TABLE IF NOT EXISTS Node(id SERIAL, workspace INT, label STRING, __id SERIAL, PRIMARY KEY(__id) );
            CREATE NODE TABLE IF NOT EXISTS Predicate(id SERIAL, workspace INT, label STRING, __id SERIAL, PRIMARY KEY (__id));
            CREATE REL TABLE IF NOT EXISTS Triple(FROM Node TO Node, id INT64);"
        ).unwrap();
    }
}
