use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use std::sync::Arc;
use super::resolvers::QueryRoot;
use cherenkov_db::RadiationDatabase;

pub type CherenkovSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn build_schema(db: Arc<RadiationDatabase>) -> anyhow::Result<CherenkovSchema> {
    Ok(Schema::build(
        QueryRoot,
        EmptyMutation,
        EmptySubscription,
    )
    .data(db)
    .finish())
}
