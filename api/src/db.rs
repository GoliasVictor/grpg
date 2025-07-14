use kuzu::{ Connection , Value, QueryResult};
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};
use crate::endpoints::nodes::GraphDirection;
use crate::endpoints::table::Filter;

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

pub trait TryFromValue : Sized{
    fn from_value(value: &Value) -> Option<Self>;
}

impl<T> TryFromValue for T
where
    Value: TryCast<T>,
{
    fn from_value(value: &Value) -> Option<T> {
        value.try_cast().ok()
    }
}

pub trait QueryResultUtil {
    fn single<K: TryFromValue>(self) -> Option<K>;
}
impl QueryResultUtil for QueryResult<'_> {
    fn single<K: TryFromValue>(self) -> Option<K> {
        let row = self.into_iter().next()?;
        let value = row.get(0)?;
        K::from_value(value)
    }
}
pub fn create_db(conn: &Connection) {
    let _ = conn.query(
        "CREATE NODE TABLE IF NOT EXISTS Node(id SERIAL, label STRING, PRIMARY KEY (id));
        CREATE NODE TABLE IF NOT EXISTS Predicate(id SERIAL, label STRING, PRIMARY KEY (id));
        CREATE REL TABLE IF NOT EXISTS Triple(FROM Node TO Node, id INT64);"
    );
}



#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct ColumnFilter {
    pub direction: Option<GraphDirection>,
    pub predicate_id: Option<i32>,
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct ColumnDefinition {
    pub id: i32,
    pub filter: ColumnFilter,
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct TableDefinition {
    pub label: String,
    pub filter: Filter,
    pub columns: Vec<ColumnDefinition>,
}
#[derive(Deserialize, Serialize)]
pub struct StoreData {
    pub tables : HashMap<i32, TableDefinition>,
}

#[derive(Deserialize, Serialize)]
pub struct Store(Mutex<()>);

impl Store {
    pub fn new() -> Self {
        Store(Mutex::new(()))
    }

    fn read(&self) -> StoreData {
        let f = std::fs::File::open("graph.yaml").unwrap();
        return serde_yaml::from_reader(f).unwrap();
    }
    fn save(&self, store: StoreData) -> (){
        let d = serde_yaml::to_string(&store).unwrap();
        std::fs::write("graph.yaml", d).unwrap();
    }


    pub fn set_table(&self, id: i32, table: TableDefinition) {
        let _lock = self.0.lock().unwrap();
        let mut store = self.read();
        store.tables.insert(id, table);
        self.save(store);
    }
    pub fn get_table(&self, id: i32) -> Option<TableDefinition> {
        let mut store = self.read();
        store.tables.remove(&id)
    }

    pub fn get_tables(&self) -> HashMap<i32, TableDefinition> {
        self.read().tables
    }

    pub fn add_table(&self, table: TableDefinition) -> i32 {
        let _lock = self.0.lock().unwrap();
        let mut store = self.read();
        let next_id = store.tables.keys().max().map_or(1, |max_id| max_id + 1);
        store.tables.insert(next_id, table);
        self.save(store);
        next_id
    }
}

