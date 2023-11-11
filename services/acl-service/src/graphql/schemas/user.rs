use async_graphql::{ComplexObject, Enum, InputObject, SimpleObject};
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::auth::oauth::OAuthClientName;

#[derive(Clone, Debug, Serialize, Deserialize, Enum, Copy, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "UserInput")]
#[graphql(complex)]
pub struct User {
    #[graphql(skip)]
    pub id: Option<Thing>,
    pub user_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub gender: Gender,
    pub dob: String,
    pub email: String,
    pub country: String,
    pub phone: String,
    #[graphql(secret)]
    pub password: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[graphql(skip)]
    pub roles: Option<Vec<Thing>>,
    pub oauth_client: Option<OAuthClientName>,
}

#[ComplexObject]
impl User {
    async fn id(&self) -> String {
        self.id.as_ref().map(|t| &t.id).expect("id").to_raw()
    }

    async fn full_name(&self) -> String {
        format!(
            "{} {} {}",
            self.first_name,
            self.middle_name.as_ref().unwrap_or(&"".to_string()),
            self.last_name
        )
    }

    async fn age(&self) -> u32 {
        // calculate age from &self.dob
        let dob = DateTime::parse_from_rfc3339(&self.dob).expect("Invalid date format");
        let from_ymd = NaiveDate::from_ymd_opt(dob.year(), dob.month0(), dob.day0()).unwrap();
        let today = Utc::now().date_naive();
        today.years_since(from_ymd).unwrap()
    }

    async fn roles(&self) -> Vec<String> {
        // implement same as for id, only that this time we are returning a vector of Thing
        self.roles
            .as_ref()
            .map(|t| t.iter().map(|t| t.id.to_raw()).collect())
            .unwrap_or(vec![])
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "UserLoginsInput")]
pub struct UserLogins {
    pub user_name: Option<String>,
    #[graphql(secret)]
    pub password: Option<String>,
    pub oauth_client: Option<OAuthClientName>,
}

impl UserLogins {
    pub fn transformed(&self) -> Self {
        let (user_name, password, oauth_client) =
            if self.password.is_some() && self.user_name.is_some() {
                (self.user_name.clone(), self.password.clone(), None)
            } else {
                (None, None, self.oauth_client)
            };

        UserLogins {
            user_name,
            password,
            oauth_client,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct AuthUrl {
    pub url: String,
}
