mod auth;
mod database;
mod graphql;

use core::panic;
use std::sync::Arc;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
// use auth::oauth::OAuthClientInstance;
use axum::{
    extract::{Extension, Query as AxumQuery},
    headers::Cookie,
    http::{HeaderValue, HeaderMap},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, TypedHeader,
};

use graphql::resolvers::query::Query;
use hyper::{Method, Server};
use oauth2::{
    basic::BasicTokenType,
    reqwest::async_http_client,
    AuthorizationCode, EmptyExtraTokenFields,
    StandardTokenResponse, PkceCodeVerifier
};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Result, Surreal};
use tower_http::cors::CorsLayer;

use graphql::resolvers::mutation::Mutation;

use crate::auth::oauth::{initiate_auth_code_grant_flow, OAuthClientName};

type MySchema = Schema<Query, Mutation, EmptySubscription>;

// type SharedState = Arc<RwLock<AppState>>;

async fn graphql_handler(
    schema: Extension<MySchema>,
    db: Extension<Arc<Surreal<Client>>>,
    // state: Extension<SharedState>,
    headers: HeaderMap,
    req: GraphQLRequest,
    
) -> GraphQLResponse {
    let mut request = req.0;
    request = request.data(db.clone());
    // request = request.data(state.clone());
    request = request.data(headers.clone());
    schema.execute(request).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

// #[derive(Clone)]
// pub struct AppState {
//     pub oauth_client: Option<OAuthClientInstance>,
//     pub csrf_state: Option<String>,
// }

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct Params {
    code: Option<String>,
    state: Option<String>,
}

// client agnostic oauth handler
async fn oauth_handler(
    params: AxumQuery<Params>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Json<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    // println!("params: {:?}", params.0);
    // get the csrf state from the cookie
    // Extract the csrf_state, oauth_client, pkce_verifier cookies
    
    let oauth_client_name = cookies
        .get("oauth_client")
        .unwrap_or_else(|| panic!("OAuth client name cookie not found"));
    let pcke_verifier_secret = cookies
        .get("k")
        .unwrap_or_else(|| panic!("OAuth client name cookie not found"));
    let csrf_state = cookies
        .get("j")
        .unwrap_or_else(|| panic!("CSRF state cookie not found"));

    println!("pcke_verifier_secret: {:?}", pcke_verifier_secret);
    println!("params.0.state.unwrap(): {:?}", &params.0.clone().state.unwrap());
    println!("csrf_state: {:?}", csrf_state);
    if params.0.state.unwrap() != csrf_state {
        panic!("CSRF token mismatch! Aborting request. Might be a hacker 🥷🏻!");
    }

    // We need to get the same client instance that we used to generate the auth url. Hence the cookies.
    let oauth_client =
        initiate_auth_code_grant_flow(OAuthClientName::from_str(oauth_client_name)).await;

    // Generate a PKCE verifier using the secret.
    let pkce_verifier = PkceCodeVerifier::new(pcke_verifier_secret.to_string());
    let auth_code = AuthorizationCode::new(params.0.code.clone().unwrap());

    // Now you can trade it for an access token.
    let token_result = oauth_client
        .exchange_code(auth_code)
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .unwrap();

    Json(token_result)
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Arc::new(database::connection::create_db_connection().await.unwrap());
    // let state = Arc::new(RwLock::new(AppState {
    //     oauth_client: None,
    //     csrf_state: None,
    // }));

    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    println!("GraphiQL IDE: http://localhost:3001");

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/oauth/callback", get(oauth_handler))
        .layer(Extension(schema))
        .layer(Extension(db))
        // .layer(Extension(state))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                .allow_methods(vec![Method::GET, Method::POST]),
        );

    Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
