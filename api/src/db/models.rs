use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};
use crate::endpoints::nodes::GraphDirection;
use crate::endpoints::table::Filter;


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
