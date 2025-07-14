mod db;
mod endpoints;
use actix_web::{middleware::Logger, App, HttpServer, web::Data};
use actix_cors::Cors;
use std::{env, error::Error};
use utoipa::OpenApi;
use utoipa_actix_web::{service_config::ServiceConfig, AppExt};
use utoipa_rapidoc::RapiDoc;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use kuzu::{ Connection, Database, SystemConfig };
use std::{
    sync::{Arc}
};

pub struct AppState {
    db: Arc<Database>,
    store: Arc<db::Store>,
}
impl AppState {
    fn establish_connection(&self) -> Connection {
        Connection::new(&self.db).unwrap()
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
    let db = Database::new("./demo_db", SystemConfig::default()).unwrap();
    let conn = Connection::new(&db).unwrap();
    db::create_db(&conn);
    drop(conn);

    let app_data = Data::new(AppState {
        db: Arc::new(db),
        store: Arc::new(db::Store::new()),
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
                    .service(endpoints::predicates::get_predicates)
                    .service(endpoints::predicates::post_predicate)
                    .service(endpoints::nodes::post_node)
                    .service(endpoints::nodes::get_node)
                    .service(endpoints::nodes::delete_node)
                    .service(endpoints::nodes::put_node)
                    .service(endpoints::triples::post_triple)
                    .service(endpoints::triples::delete_triple)
                    .service(endpoints::triples::get_triples)
                    .service(endpoints::table::full_table)
                    .service(endpoints::table::get_table)
                    .service(endpoints::table::post_table)
                    .service(endpoints::table::get_tables)
                    ;
            })
            .openapi_service(|api| {
                RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc")
            })
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app()
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
