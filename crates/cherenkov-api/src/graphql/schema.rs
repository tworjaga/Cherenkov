use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use super::resolvers::QueryRoot;

pub type CherenkovSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn build_schema() -> anyhow::Result<CherenkovSchema> {
    Ok(Schema::new(
        QueryRoot,
        EmptyMutation,
        EmptySubscription,
    ))
}
