use async_graphql::{Schema, MergedObject, MergedSubscription};
use std::sync::Arc;
use super::resolvers::QueryRoot;
use super::model_management::{ModelQueryRoot, ModelMutationRoot, TrainingJobQueryRoot, DataSourceQueryRoot};
use super::subscription::SubscriptionRoot;
use cherenkov_db::RadiationDatabase;
use cherenkov_ml::ModelRegistry;

#[derive(MergedObject, Default)]
pub struct FullQueryRoot(QueryRoot, ModelQueryRoot, TrainingJobQueryRoot, DataSourceQueryRoot);

#[derive(MergedObject, Default)]
pub struct FullMutationRoot(ModelMutationRoot);

#[derive(MergedSubscription, Default)]
pub struct FullSubscriptionRoot(SubscriptionRoot);

pub type CherenkovSchema = Schema<FullQueryRoot, FullMutationRoot, FullSubscriptionRoot>;

pub async fn build_schema(
    db: Arc<RadiationDatabase>,
    model_registry: Arc<ModelRegistry>,
) -> anyhow::Result<CherenkovSchema> {
    Ok(Schema::build(
        FullQueryRoot::default(),
        FullMutationRoot::default(),
        FullSubscriptionRoot::default(),
    )
    .data(db)
    .data(model_registry)
    .finish())
}
