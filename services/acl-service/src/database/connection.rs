use dotenvy::dotenv;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root, Result};

pub async fn create_db_connection() -> Result<Surreal<Client>> {
    dotenv().ok();
    println!("Creating Surreal database connection...");
    let db_host = std::env::var("DATABASE_HOST").expect("DB_HOST not set");
    let db_port = std::env::var("DATABASE_PORT").expect("DB_PORT not set");
    let db_user: String = std::env::var("DATABASE_USER").expect("DB_USER not set");
    let db_password: String = std::env::var("DATABASE_PASSWORD").expect("DB_PASSWORD not set");
    let db_name: String = std::env::var("DATABASE_NAME").expect("DB_NAME not set");
    let db_namespace: String = std::env::var("DATABASE_NAMESPACE").expect("DB_NAMESPACE not set");

    let db = Surreal::new::<Ws>(format!("{}:{}", db_host, db_port).as_str()).await?;

    db.signin(Root {
        username: db_user.as_str(),
        password: db_password.as_str(),
    })
    .await?;

    // Select a specific namespace and database
    db.use_ns(db_namespace.as_str()).use_db(db_name.as_str()).await?;

    Ok(db)
}