use std::sync::Arc;

use async_graphql::{Object, Context, Result, Error};
use axum::Extension;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::graphql::schemas::user::User;

pub struct Query;

#[Object]
impl Query {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db = ctx.data::<Extension<Arc<Surreal<Client>>>>().unwrap();
        
        let response = db.select("user").await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(response)
    }
}