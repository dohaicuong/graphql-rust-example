use actix_web::{App, HttpServer, web, guard};
use actix_cors::Cors;

mod graphql;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Playground: http://localhost:8000");

    let conn = db::pool::get_db_connection().await.unwrap();
    db::migration::migrate(&conn).await;
    
    let schema = graphql::schema::build_schema(
        graphql::schema::ContextState { conn }
    );
    graphql::utils::write_schema(&schema)?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(graphql::routes::index))
            .service(web::resource("/").guard(guard::Get()).to(graphql::routes::index_playground))
            .wrap(Cors::permissive())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
