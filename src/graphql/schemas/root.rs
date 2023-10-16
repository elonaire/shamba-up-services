pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn value(&self) -> i32 {
        100
    }
}