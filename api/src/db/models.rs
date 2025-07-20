use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};


#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserData {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Node {
    pub node_id: i32,
    pub label: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Predicate {
    pub id: i32,
    pub label: String,
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
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

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct Filter {
    pub node_id: Option<i32>,
    pub predicate: Option<i32>,
    pub direction: Option<GraphDirection>,
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
