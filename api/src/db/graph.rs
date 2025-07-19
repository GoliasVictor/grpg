mod triples;
mod nodes;
mod predicates;
use crate::endpoints::triples::Triple;
use crate::db::models::{
    Node,
    Predicate,
    TableDefinition,
    RowResponse
};
use kuzu::{
    Connection
};
mod table;
pub struct GraphManager<'a> {
    pub conn: Connection<'a>,
    pub setting: i32
}

impl<'a> GraphManager<'a> {
    pub fn triple_create(&self, triple: Triple) {
        triples::triple_create(&self.conn, self.setting, triple)
    }
    pub fn triple_delete(&self, triple: Triple) {
        triples::triple_delete(&self.conn, self.setting, triple)
    }
    pub fn triple_all(&self) -> Vec<Triple> {
        triples::triple_all(&self.conn, self.setting)
    }
    pub fn node_create(&self, label: String) -> i32 {
        nodes::node_create(&self.conn, self.setting, label)
    }
    pub fn node_all(&self,) -> Vec<Node> {
        nodes::node_all(&self.conn, self.setting)
    }
    pub fn node_update(&self, node_id: i32, label: String) -> Node {
        nodes::node_update(&self.conn, self.setting, node_id, label)
    }
    pub fn node_delete(&self, node_id: i32) {
        nodes::node_delete(&self.conn, self.setting, node_id)
    }
    pub fn predicate_all(&self) -> Vec<Predicate> {
        predicates::predicate_all(&self.conn, self.setting)
    }
    pub fn predicate_create(&self, label: &str ) -> Predicate {
        predicates::predicate_create(&self.conn, self.setting, label)
    }
    pub async fn table_rows(&self, table_def: TableDefinition) -> Vec<RowResponse> {
        table::table_rows(&self.conn, self.setting, table_def).await
    }
}


