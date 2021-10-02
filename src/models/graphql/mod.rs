pub mod nhapi;
pub mod nhentai;

use async_graphql::{ 
    Schema,
    MergedObject, 
    EmptyMutation, 
    EmptySubscription, 
    extensions::ApolloTracing 
};

use nhentai::NHentaiQueryRoot;

#[derive(MergedObject, Default)]
pub struct Query(NHentaiQueryRoot);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(
        Query::default(),
        EmptyMutation,
        EmptySubscription
    )
    .extension(ApolloTracing)
    .finish()
}