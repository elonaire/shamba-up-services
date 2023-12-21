use std::{sync::Arc, time::{UNIX_EPOCH, SystemTime, Duration}};

use async_graphql::{Context, Error, Object, Result};
use axum::Extension;
use hmac::{Hmac, Mac};
use hyper::header::SET_COOKIE;
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    auth::oauth::{initiate_auth_code_grant_flow, navigate_to_redirect_url},
    graphql::schemas::{
        role::SystemRole,
        user::{AuthDetails, User, UserLogins, JWTClaimBTreeMapItem},
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
                oauth_client: None,
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

    async fn sign_in(
        &self,
        ctx: &Context<'_>,
        raw_user_details: UserLogins,
    ) -> Result<AuthDetails> {
        let user_details = raw_user_details.transformed();
        match user_details.oauth_client {
            Some(oauth_client) => {
                let oauth_client_instance = initiate_auth_code_grant_flow(oauth_client).await;
                let redirect_url =
                    navigate_to_redirect_url(oauth_client_instance, ctx, oauth_client).await;
                Ok(AuthDetails {
                    url: Some(redirect_url),
                    token: None,
                })
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
                    Some(user) => {
                        let password_match = bcrypt::verify(
                            &user_details.password.unwrap(),
                            response.clone().unwrap().password.as_str(),
                        )
                        .unwrap();

                        if password_match {
                            // Generate JWT access token
                            let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
                            let key: Hmac<Sha256> =
                                Hmac::new_from_slice(secret.as_str().as_bytes()).unwrap();
                            let mut claims: BTreeMap<&str, JWTClaimBTreeMapItem> = BTreeMap::new();
                            let expiry_duration = Duration::from_secs(15 * 60); // minutes by 60 seconds
                            let current_time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");
                            let expiry_time = current_time.as_secs() + expiry_duration.as_secs();
                            claims.insert("exp", JWTClaimBTreeMapItem::Integer(expiry_time));
                            claims.insert(
                                "sub",
                                JWTClaimBTreeMapItem::String(user.id.as_ref().map(|t| &t.id).expect("id").to_raw()),
                            );

                            // Generate JWT refresh token
                            let refresh_token_key: Hmac<Sha256> = Hmac::new_from_slice(
                                std::env::var("JWT_REFRESH_SECRET")
                                    .expect("JWT_REFRESH_SECRET not set")
                                    .as_str()
                                    .as_bytes(),
                            ).unwrap();

                            let mut refresh_token_claims: BTreeMap<&str, JWTClaimBTreeMapItem> = BTreeMap::new();
                            let refresh_token_expiry_duration = Duration::from_secs(30 * 24 * 60 * 60); // minutes by 60 seconds
                            let refresh_token_current_time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");
                            let refresh_token_expiry_time =
                                refresh_token_current_time.as_secs() + refresh_token_expiry_duration.as_secs();
                            refresh_token_claims.insert(
                                "exp",
                                JWTClaimBTreeMapItem::Integer(refresh_token_expiry_time).into(),
                            );

                            refresh_token_claims.insert(
                                "sub",
                                JWTClaimBTreeMapItem::String(user.id.as_ref().map(|t| &t.id).expect("id").to_raw()).into(),
                            );

                            let refresh_token_str = refresh_token_claims
                                .sign_with_key(&refresh_token_key)
                                .unwrap();

                            // set refresh token in HttpOnly cookie after symetric encryption
                            
                            let token_str = claims.sign_with_key(&key).unwrap();

                            ctx.insert_http_header(
                                SET_COOKIE,
                                format!("oauth_client="),
                            );

                            ctx.append_http_header(
                                SET_COOKIE,
                                format!(
                                    "t={}; Max-Age={}",
                                    refresh_token_str,
                                    refresh_token_expiry_duration.as_secs()
                                ),
                            );
                            Ok(AuthDetails {
                                token: Some(token_str),
                                url: None,
                            })
                        } else {
                            Err(Error::new("Invalid username or password"))
                        }
                    }
                    None => Err(Error::new("Invalid username or password")),
                }
            }
        }
    }

    async fn sign_out(&self, ctx: &Context<'_>) -> Result<bool> {
        // Clear the refresh token cookie
        ctx.insert_http_header(
            SET_COOKIE,
            format!("t=; Max-Age=0"),
        );
        Ok(true)
    }
}
