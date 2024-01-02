use async_graphql::Object;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self) -> bool {
        true
    }
} 
