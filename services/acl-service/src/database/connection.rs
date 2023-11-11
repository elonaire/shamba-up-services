use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root, Result};

pub async fn create_db_connection() -> Result<Surreal<Client>> {
    println!("Creating Surreal database connection...");
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace and database
    db.use_ns("shamba_up").use_db("acl").await?;

    Ok(db)
}