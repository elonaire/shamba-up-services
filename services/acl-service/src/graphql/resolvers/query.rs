use std::{collections::BTreeMap, env, sync::Arc};

use async_graphql::{Context, Error, Object, Result};
use axum::{
    http::{header::COOKIE, HeaderMap},
    Extension,
};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use lib::utils::cookie_parser::parse_cookies;
use reqwest::{Client as ReqWestClient, header::HeaderMap as ReqWestHeaderMap};
use sha2::Sha256;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    auth::oauth::{self, OAuthClientName},
    graphql::schemas::user::{AuthStatus, DecodedGithubOAuthToken, DecodedGoogleOAuthToken, User},
};

pub struct Query;

#[Object]
impl Query {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db = ctx.data::<Extension<Arc<Surreal<Client>>>>().unwrap();

        let response = db
            .select("user")
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(response)
    }

    async fn check_auth(&self, ctx: &Context<'_>) -> Result<AuthStatus> {
        dotenv().ok();
        let jwt_secret =
            env::var("JWT_SECRET").expect("Missing the JWT_SECRET environment variable.");
        let jwt_refresh_secret = env::var("JWT_REFRESH_SECRET")
            .expect("Missing the JWT_REFRESH_SECRET environment variable.");
        // Process request headers as needed
        match ctx.data_opt::<HeaderMap>() {
            Some(headers) => {
                // Check if Authorization header is present
                match headers.get("Authorization") {
                    Some(token) => {
                        // Check if Cookie header is present
                        match headers.get(COOKIE) {
                            Some(cookie_header) => {
                                let cookies_str = cookie_header
                                    .to_str()
                                    .map_err(|_| "Invalid cookie format")?;
                                let cookies = parse_cookies(cookies_str);

                                // Check if oauth_client cookie is present
                                match cookies.get("oauth_client") {
                                    Some(oauth_client) => {
                                        if oauth_client.is_empty() {
                                            let key: Hmac<Sha256> = Hmac::new_from_slice(
                                                jwt_secret.as_str().as_bytes(),
                                            )
                                            .unwrap();
                                            let token_str =
                                                token.to_str().unwrap().strip_prefix("Bearer ");
                                            let mut _claims: Result<
                                                BTreeMap<String, String>,
                                                jwt::Error,
                                            > = Ok(BTreeMap::new());

                                            // Check if token is present and valid
                                            match token_str {
                                                Some(token_str) => {
                                                    _claims = token_str.verify_with_key(&key);

                                                    match &mut _claims {
                                                        Ok(_) => {
                                                            // Token verification successful
                                                            return Ok(AuthStatus {
                                                                is_auth: true,
                                                            });
                                                        }
                                                        Err(_err) => {
                                                            // Token verification failed, check if refresh token is present
                                                            match cookies.get("refresh_token") {
                                                                Some(refresh_token) => {
                                                                    let jwt_refresh_key: Hmac<
                                                                        Sha256,
                                                                    > = Hmac::new_from_slice(
                                                                        jwt_refresh_secret
                                                                            .as_str()
                                                                            .as_bytes(),
                                                                    )
                                                                    .unwrap();

                                                                    let mut _refresh_claims: Result<
                                                                        BTreeMap<String, String>,
                                                                        jwt::Error,
                                                                    > = Ok(BTreeMap::new());

                                                                    _refresh_claims = refresh_token
                                                                        .verify_with_key(
                                                                            &jwt_refresh_key,
                                                                        );

                                                                    match _refresh_claims {
                                                                        Ok(_) => {
                                                                            // Refresh token verification successful, issue new access token
                                                                            // call sign_in mutation

                                                                            return Ok(
                                                                                AuthStatus {
                                                                                    is_auth: false,
                                                                                },
                                                                            );
                                                                        }
                                                                        Err(_err) => {
                                                                            // Refresh token verification failed
                                                                            return Err(Error::new(
                                                                                        "Not Authorized!",
                                                                                    ));
                                                                        }
                                                                    }
                                                                }
                                                                None => Err(Error::new(
                                                                    "Not Authorized!",
                                                                )),
                                                            }
                                                            // return Err(Error::new("Not Authorized!"));
                                                        }
                                                    }
                                                }
                                                None => Err(Error::new("Invalid header(s)")),
                                            }
                                        } else {
                                            let oauth_client_name =
                                                oauth::OAuthClientName::from_str(oauth_client);

                                            match oauth_client_name {
                                                OAuthClientName::Google => {
                                                    // make a request to google oauth server to verify the token
                                                    let response =
                                                        reqwest::get(format!("https://oauth2.googleapis.com/tokeninfo?access_token={}", token.to_str().unwrap().strip_prefix("Bearer ").unwrap()).as_str())
                                                            .await?
                                                            .json::<DecodedGoogleOAuthToken>()
                                                            .await?;
                                                    println!("response: {:?}", response);

                                                    return Ok(AuthStatus { is_auth: true });
                                                }
                                                OAuthClientName::Github => {
                                                    // make a request to github oauth server to verify the token
                                                    let client = ReqWestClient::new();

                                                    let mut req_headers = ReqWestHeaderMap::new();
                                                    req_headers.insert(
                                                        "Authorization",
                                                        token.to_owned(),
                                                    );

                                                    req_headers.insert(
                                                        "Accept",
                                                            "application/vnd.github+json".parse().unwrap(),
                                                    );

                                                    req_headers.insert(
                                                        "X-GitHub-Api-Version",
                                                            "2022-11-28".parse().unwrap(),
                                                    );

                                                    println!("req_headers: {:?}", req_headers);

                                                    let response = client
                                                        .get("https://api.github.com/user")
                                                        .headers(req_headers)
                                                        .send()
                                                        .await?
                                                        .json::<DecodedGithubOAuthToken>()
                                                        .await?;

                                                    println!("response: {:?}", response);

                                                    return Ok(AuthStatus { is_auth: true });
                                                }
                                            }
                                        }
                                    }
                                    None => Err(Error::new("Not Authorized!")),
                                }
                            }
                            None => Err(Error::new("Not Authorized!")),
                        }
                    }
                    None => Err(Error::new("Not Authorized!")),
                }
            }
            None => Err(Error::new("Invalid request!")),
        }
    }
}
