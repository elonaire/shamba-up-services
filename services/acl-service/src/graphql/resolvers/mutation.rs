use std::sync::Arc;

use async_graphql::{Context, Error, Object, Result};
use axum::Extension;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    auth::oauth::{initiate_auth_code_grant_flow, navigate_to_redirect_url},
    graphql::schemas::{
        role::SystemRole,
        user::{AuthUrl, User, UserLogins},
    },
};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn sign_up(&self, ctx: &Context<'_>, mut user: User) -> Result<Vec<User>> {
        user.password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST).unwrap();
        user.dob = chrono::DateTime::parse_from_rfc3339(&user.dob)
            .unwrap()
            .to_rfc3339();

        // User signup
        let db = ctx.data::<Extension<Arc<Surreal<Client>>>>().unwrap();
        let response: Vec<User> = db
            .create("user")
            .content(User {
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                updated_at: Some(chrono::Utc::now().to_rfc3339()),
                ..user
            })
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(response)
    }

    async fn create_user_role(
        &self,
        ctx: &Context<'_>,
        role: SystemRole,
    ) -> Result<Vec<SystemRole>> {
        let db = ctx.data::<Extension<Arc<Surreal<Client>>>>().unwrap();
        let response = db
            .create("role")
            .content(SystemRole { ..role })
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(response)
    }

    async fn sign_in(&self, ctx: &Context<'_>, raw_user_details: UserLogins) -> Result<AuthUrl> {
        let user_details = raw_user_details.transformed();
        match user_details.oauth_client {
            Some(oauth_client) => {
                let oauth_client_instance = initiate_auth_code_grant_flow(oauth_client).await;
                let redirect_url = navigate_to_redirect_url(oauth_client_instance, ctx, oauth_client).await;
                Ok(AuthUrl { url: redirect_url })
            }
            None => {
                let db = ctx.data::<Extension<Arc<Surreal<Client>>>>().unwrap();
                    let db_query = format!(
                        "SELECT * FROM type::table($table) WHERE email = '{}' OR user_name = '{}' LIMIT 1",
                        &user_details.user_name.clone().unwrap(),
                        &user_details.user_name.clone().unwrap()
                    );
            
                    let mut result = db.query(db_query).bind(("table", "user")).await?;
                    // Get the first result from the first query
                    let response: Option<User> = result.take(0)?;
            
                    match &response {
                        Some(_user) => {
                            let password_match = bcrypt::verify(
                                &user_details.password.unwrap(),
                                response.clone().unwrap().password.as_str(),
                            )
                            .unwrap();
            
                            if password_match {
                                //TODO: Generate JWT token
            
                                Ok(AuthUrl { url: "http://localhost:3001".to_string() })
                            } else {
                                Err(Error::new("Invalid username or password"))
                            }
                        }
                        None => Err(Error::new("Invalid username or password")),
                    }
            }
        }
    }
}
