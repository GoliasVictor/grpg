mod db;
mod endpoints;
use actix_web::{middleware::Logger, App, HttpServer, web::Data};
use actix_cors::Cors;
use std::{env, error::Error};
use utoipa::OpenApi;
use utoipa_actix_web::{service_config::ServiceConfig, AppExt};
use utoipa_rapidoc::RapiDoc;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use std::{
    sync::{Arc}
};
use crate::db::graphdb::GraphManager;
use crate::db::reldb::Store;
use crate::db::graphdb::GraphDatabase;

pub struct AppState {
    db: Arc<GraphDatabase>,
    store: Arc<Store>,
}
impl AppState {
    pub fn graph(&self, workspace_id: i32) -> GraphManager {
        GraphManager {
            conn: self.db.connection(),
            workspace: workspace_id
        }
    }
}
#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();
    unsafe {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    }
    #[derive(OpenApi)]
    struct ApiDoc;
    let graph_db = db::graphdb::GraphDatabase::new("./demo_db");
    graph_db.startup();

    let app_data = Data::new(AppState {
        db: Arc::new(graph_db),
        store: Arc::new(Store::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .configure(|config: &mut ServiceConfig| {
                config
                    .service(endpoints::users::post_user)
                    .service(endpoints::users::get_users)
                    .service(endpoints::users::get_user_by_id)
                    .service(endpoints::workspaces::post_workspace)
                    .service(endpoints::workspaces::get_workspaces)
                    .service(endpoints::workspaces::get_workspace_by_id)
                    .service(endpoints::predicates::get_predicates)
                    .service(endpoints::predicates::post_predicate)
                    .service(endpoints::nodes::post_node)
                    .service(endpoints::nodes::get_node)
                    .service(endpoints::nodes::delete_node)
                    .service(endpoints::nodes::put_node)
                    .service(endpoints::triples::post_triple)
                    .service(endpoints::triples::delete_triple)
                    .service(endpoints::triples::get_triples)
                    .service(endpoints::table::put_table)
                    .service(endpoints::table::get_table)
                    .service(endpoints::table::get_table)
                    .service(endpoints::table::post_table)
                    .service(endpoints::table::get_tables)
                    .service(endpoints::table::delete_table)
                    .service(endpoints::hooks::github_webhook)
                    ;
            })
            .openapi_service(|api| {
                RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc")
            })
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app()
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
