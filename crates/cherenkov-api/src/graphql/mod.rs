pub mod schema;
pub mod resolvers;
pub mod subscription;
pub mod model_management;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;

pub async fn handler(
    schema: Extension<schema::CherenkovSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[allow(dead_code)]
pub async fn subscription_handler(
    schema: Extension<schema::CherenkovSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
