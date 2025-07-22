pub mod reldb;
pub mod models;
pub mod graphdb;
use kuzu::{ Connection, Value, QueryResult };

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
pub fn create_db(conn: &Connection) {
    let _ = conn.query(
        "CREATE NODE TABLE IF NOT EXISTS Node(id SERIAL, workspace INT, label STRING, __id SERIAL, PRIMARY KEY(__id) );
        CREATE NODE TABLE IF NOT EXISTS Predicate(id SERIAL, workspace INT, label STRING, __id SERIAL, PRIMARY KEY (__id));
        CREATE REL TABLE IF NOT EXISTS Triple(FROM Node TO Node, id INT64);"
    ).unwrap();
}
